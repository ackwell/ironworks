use std::io::{Read, Seek};

use binrw::{binread, BinRead, BinResult, ReadOptions};
use getset::{CopyGetters, Getters};

use super::command::{
	AddCommand, DeleteCommand, ExpandCommand, FileOperationCommand, HeaderUpdateCommand,
	IndexUpdateCommand, PatchInfoCommand, TargetInfoCommand,
};

/// A chunk of a patch file, encapsulating metadata about the containing file,
/// or a single task that should be performed.
#[binread]
#[br(big, import(chunk_size: u32))]
#[derive(Debug)]
pub enum Chunk {
	/// Metadata about the .patch file and information it contains.
	#[br(magic = b"FHDR")]
	FileHeader(FileHeaderChunk),

	/// An option key-value pair that should be applied while reading remaining chunks.
	#[br(magic = b"APLY")]
	Apply(ApplyChunk),

	/// Create a new directory.
	#[br(magic = b"ADIR")]
	AddDirectory(AddDirectoryChunk),

	/// Delete an empty folder.
	#[br(magic = b"DELD")]
	DeleteDirectory(DeleteDirectoryChunk),

	/// Extension chunk to perform operations on a SqPack-based game install.
	#[br(magic = b"SQPK")]
	SqPack(#[br(args(chunk_size))] SqPackChunk),

	/// The last chunk in a .patch file, signifying no further chunks should be read.
	#[br(magic = b"EOF_")]
	EndOfFile,
}

/// Metadata about the .patch file and information it contains.
#[binread]
#[br(big)]
#[derive(Debug, Getters, CopyGetters)]
pub struct FileHeaderChunk {
	// unk1: u16
	/// Version of the patch format.
	#[br(pad_before = 2)]
	#[get_copy = "pub"]
	version: u8,

	// unk2: u8
	/// Type of the patch file.
	///
	/// NOTE: This value is likely untrustworthy, and frequently does not match
	/// the type of patch indicated by the patch's file name. Take with a grain
	/// of salt.
	#[br(pad_before = 1)]
	#[get_copy = "pub"]
	patch_kind: PatchKind,

	///
	#[get_copy = "pub"]
	entry_files: u32,

	/// Version 3 specific fields.
	#[br(if(version == 3))]
	#[get = "pub"]
	v3: Option<FileHeaderV3>,
}

#[binread]
#[br(big)]
#[derive(Debug, Clone, Copy)]
pub enum PatchKind {
	#[br(magic = b"DIFF")]
	Diff,

	#[br(magic = b"HIST")]
	Hist,
}

/// Additional fields available in file header chunks in version 3 of the format.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct FileHeaderV3 {
	///
	add_directories: u32,
	///
	delete_directories: u32,

	// wtaf?
	#[br(temp)]
	delete_data_1: u32,
	#[br(temp)]
	delete_data_2: u32,
	///
	#[br(calc = u64::from(delete_data_1) | u64::from(delete_data_2) << 32)]
	delete_data: u64,

	///
	minor_version: u32,
	///
	repository_name: u32, // crc'd? how is this a name
	///
	commands: u32,
	///
	sqpack_add_commands: u32,
	///
	sqpack_delete_commands: u32,
	///
	sqpack_expand_commands: u32,
	///
	sqpack_header_commands: u32,
	///
	sqpack_file_commands: u32,
}

/// An option key-value pair that should be applied while reading remaining chunks.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct ApplyChunk {
	/// Option key to configure.
	option: OptionKind,

	// unk1: u32,
	/// Value to set for the option. For both known options, a non-zero `value`
	/// represents `true`.
	#[br(pad_before = 4)]
	value: u32,
	// unk2: [u8; 4],
}

#[allow(missing_docs)]
#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Clone, Copy)]
pub enum OptionKind {
	IgnoreMissing = 1,
	IgnoreMismatch = 2,
}

/// Create a new directory.
#[binread]
#[br(big)]
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct AddDirectoryChunk {
	#[br(temp)]
	length: u32,

	/// Path of the directory to add. Path is relative to the target folder.
	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

/// Delete an empty folder. Deleting a non-empty folder is considered an error.
#[binread]
#[br(big)]
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct DeleteDirectoryChunk {
	#[br(temp)]
	length: u32,

	/// Path of the directory to delete. Path is relative to the target folder.
	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

/// Extension chunk to perform operations on a SqPack-based game install.
#[derive(Debug)]
pub enum SqPackChunk {
	/// Write data to a file.
	Add(AddCommand),

	/// Delete data from a file.
	Delete(DeleteCommand),

	/// Expand the size of a file.
	Expand(ExpandCommand),

	/// Perform a file operation.
	FileOperation(FileOperationCommand),

	/// Update the header of a file.
	HeaderUpdate(HeaderUpdateCommand),

	/// Update an entry in a SqPack index file. This command is currently unused.
	IndexUpdate(IndexUpdateCommand),

	/// Metadata about the SqPack patch.
	PatchInfo(PatchInfoCommand),

	/// Metadata about the target SqPack install.
	TargetInfo(TargetInfoCommand),
}

// Manual BinRead implementation because of that pesky size: u32 at the start of sqpack chunks that we don't want.
impl BinRead for SqPackChunk {
	type Args = (u32,);

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		(chunk_size,): Self::Args,
	) -> BinResult<Self> {
		// NOTE: in all observed instances, this size value is equivalent to the parent size on the chunk container.
		let inner_size = u32::read_options(reader, options, ())?;
		assert!(inner_size == chunk_size);

		let pos = reader.stream_position()?;
		let magic = u8::read_options(reader, options, ())?;

		// The command is 5 bytes smaller than the chunk, due to the header read above.
		let command_size = chunk_size - 5;

		let command = match magic {
			b'A' => Self::Add(AddCommand::read_options(reader, options, ())?),
			b'D' => Self::Delete(DeleteCommand::read_options(reader, options, ())?),
			b'E' => Self::Expand(ExpandCommand::read_options(reader, options, ())?),
			b'F' => Self::FileOperation(FileOperationCommand::read_options(
				reader,
				options,
				(command_size,),
			)?),
			b'H' => Self::HeaderUpdate(HeaderUpdateCommand::read_options(reader, options, ())?),
			b'I' => Self::IndexUpdate(IndexUpdateCommand::read_options(reader, options, ())?),
			b'X' => Self::PatchInfo(PatchInfoCommand::read_options(reader, options, ())?),
			b'T' => Self::TargetInfo(TargetInfoCommand::read_options(reader, options, ())?),
			other => {
				return Err(binrw::Error::BadMagic {
					pos,
					found: Box::new(other),
				});
			}
		};

		Ok(command)
	}
}
