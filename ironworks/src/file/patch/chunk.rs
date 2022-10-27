use std::io::{Read, Seek};

use binrw::{binread, BinRead, BinResult, ReadOptions};
use getset::{CopyGetters, Getters};

use super::command::{
	AddCommand, DeleteCommand, ExpandCommand, FileOperationCommand, HeaderUpdateCommand,
	IndexUpdateCommand, PatchInfoCommand, TargetInfoCommand,
};

#[binread]
#[br(big, import(chunk_size: u32))]
#[derive(Debug)]
pub enum Chunk {
	#[br(magic = b"FHDR")]
	FileHeader(FileHeaderChunk),

	#[br(magic = b"APLY")]
	Apply(ApplyChunk),

	#[br(magic = b"ADIR")]
	AddDirectory(AddDirectoryChunk),

	#[br(magic = b"DELD")]
	DeleteDirectory(DeleteDirectoryChunk),

	#[br(magic = b"SQPK")]
	SqPack(#[br(args(chunk_size))] SqPackChunk),

	#[br(magic = b"EOF_")]
	EndOfFile,
}

#[binread]
#[br(big)]
#[derive(Debug, Getters, CopyGetters)]
pub struct FileHeaderChunk {
	// unk1: u16
	#[br(temp)]
	#[br(pad_before = 2)]
	version: u8,

	// unk2: u8
	// note don't trust this it doesn't seem to match the file name's suggestion
	#[br(pad_before = 1)]
	#[get_copy = "pub"]
	patch_kind: PatchKind,

	#[get_copy = "pub"]
	entry_files: u32,

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

#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct FileHeaderV3 {
	add_directories: u32,
	delete_directories: u32,

	// wtaf?
	#[br(temp)]
	delete_data_1: u32,
	#[br(temp)]
	delete_data_2: u32,
	#[br(calc = u64::from(delete_data_1) | u64::from(delete_data_2) << 32)]
	delete_data: u64,

	minor_version: u32,
	repository_name: u32, // crc'd? how is this a name
	commands: u32,
	sqpack_add_commands: u32,
	sqpack_delete_commands: u32,
	sqpack_expand_commands: u32,
	sqpack_header_commands: u32,
	sqpack_file_commands: u32,
}

#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct ApplyChunk {
	option: OptionKind,

	#[br(pad_before = 4)]
	// unk1: u32,
	value: u32,
	// unk2: [u8; 4],
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Clone, Copy)]
pub enum OptionKind {
	IgnoreMissing = 1,
	IgnoreMismatch = 2,
}

#[binread]
#[br(big)]
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct AddDirectoryChunk {
	#[br(temp)]
	length: u32,

	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

#[binread]
#[br(big)]
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct DeleteDirectoryChunk {
	#[br(temp)]
	length: u32,

	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

#[derive(Debug)]
pub enum SqPackChunk {
	Add(AddCommand),
	Delete(DeleteCommand),
	Expand(ExpandCommand),
	FileOperation(FileOperationCommand),
	HeaderUpdate(HeaderUpdateCommand),
	IndexUpdate(IndexUpdateCommand),
	PatchInfo(PatchInfoCommand),
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
