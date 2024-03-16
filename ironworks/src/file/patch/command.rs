use std::io::{Read, Seek, SeekFrom};

use binrw::helpers::until_eof;
use binrw::{binread, BinRead, BinResult, Endian, NullString, PosValue};
use getset::{CopyGetters, Getters};

const UNCOMPRESSED_MARKER_SIZE: u32 = 32_000;

/// Representation of a file within a SqPack file tree.
///
/// The path can be constructed with a few additional fields from other sources:
///
/// `"{main_id:02x}{sub_id:04x}.{platform}.{file_type}{maybe_file_id}"`
///
/// Where `platform` is a string, such as `"win32"`, `file_type` is `"dat"` or
/// `"index"`, and `maybe_file_id` is an empty string for indices with `file_id == 0`,
/// and otherwise equivalent to `file_id`.
#[binread]
#[br(big)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct SqPackFile {
	///
	main_id: u16,
	///
	sub_id: u16,
	///
	file_id: u32,
}

/// Write data to a file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct AddCommand {
	// unk1: [u8; 3]
	/// File to modify.
	#[br(pad_before = 3)]
	file: SqPackFile,
	/// Target file offset to start writing, in bytes.
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
	/// Size of data to copy, in bytes.
	#[br(map = |value: u32| value << 7)]
	data_size: u32,
	/// Number of bytes to blank after writing.
	#[br(map = |value: u32| value << 7)]
	delete_size: u32,

	/// Offset in bytes within the patch file to read the data from.
	#[br(map = |value: PosValue<()>| value.pos)]
	source_offset: u64,
}

/// Delete data from a file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct DeleteCommand {
	// unk1: [u8; 3]
	/// File to modify.
	#[br(pad_before = 3)]
	file: SqPackFile,
	/// Offset to start writing at.
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
	/// Number of blank bytes that that should be written.
	#[br(map = |value: u32| value << 7)]
	delete_size: u32,
}

/// Expand the size of a file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct ExpandCommand {
	// unk1: [u8; 3]
	/// File to modify.
	#[br(pad_before = 3)]
	file: SqPackFile,
	/// Offset to start writing at.
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
	/// Number of blank bytes that that should be written.
	#[br(map = |value: u32| value << 7)]
	delete_size: u32,
}

/// Perform a file operation.
#[binread]
#[derive(Debug, Getters, CopyGetters)]
#[br(big, import(command_size: u32))]
pub struct FileOperationCommand {
	#[br(temp, map = |value: PosValue<()>| value.pos)]
	command_start: u64,

	#[br(temp)]
	operation_magic: u8,

	// unk1: [u8; 2]
	/// Offset within the target file to start writing in the case of an AddFile operation.
	#[br(pad_before = 2)]
	#[get_copy = "pub"]
	target_offset: u64,

	/// Number of bytes that will be written to the target file. This will typically
	/// max out at 1,600,000 bytes - larger files will be split across multiple chunks.
	#[get_copy = "pub"]
	target_size: u64,

	#[br(temp)]
	path_length: u32,

	///
	#[get_copy = "pub"]
	repository_id: u16,

	// unk2: [u8; 2]
	/// Path of the target file within the game's directory.
	#[br(pad_before = 2)]
	#[br(pad_size_to = path_length)]
	#[get = "pub"]
	path: NullString,

	/// File operation to be performed.
	#[br(args(operation_magic, command_start, command_size))]
	#[get = "pub"]
	operation: FileOperation,
}

/// The operation that should be performed by a file operation command.
#[binread]
#[br(import(magic: u8, command_start: u64, command_size: u32))]
#[derive(Debug)]
pub enum FileOperation {
	/// Write data to the specified file. If `target_offset == 0`, the target file should be truncated before writing.
	#[br(pre_assert(magic == b'A'))]
	AddFile(
		#[br(parse_with = parse_block_headers)]
		#[br(args(command_start, command_size))]
		Vec<BlockHeader>,
	),

	// Unused?
	/// Delete the specified file.
	#[br(pre_assert(magic == b'D'))]
	DeleteFile,

	// Unused?
	/// Create the directories required to represent the specified path.
	#[br(pre_assert(magic == b'M'))]
	MakeDirTree,

	/// Remove all files for the specified repository ID.
	#[br(pre_assert(magic == b'R'))]
	RemoveAll,
}

fn parse_block_headers<R: Read + Seek>(
	reader: &mut R,
	options: Endian,
	(command_start, command_size): (u64, u32),
) -> BinResult<Vec<BlockHeader>> {
	let command_end = u64::from(command_size) + command_start;

	let mut headers = vec![];

	// Read headers while there's space left in the command's data.
	while reader.stream_position()? < command_end {
		let header = BlockHeader::read_options(reader, options, ())?;

		// Blocks can be compressed or uncompressed in source data, and are sorta-kinda aligned in typical Square fashion.
		let payload_size = header.payload_size();
		let aligned_size = (u64::from(payload_size) + 0x8F) & 0xFFFFFF80;

		// Skip over the block's payload.
		reader.seek(SeekFrom::Current(
			(aligned_size - u64::from(header.header_size))
				.try_into()
				.unwrap(),
		))?;

		headers.push(header);
	}

	Ok(headers)
}

// This is identical to the `BlockHeader` in `sqpack::file` - TODO: look into sharing.
/// Block of potentially-compressed data
#[binread]
#[br(little)] // REALLY?
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct BlockHeader {
	/// Size of this block's header. Practically always 16 bytes.
	header_size: u32,
	// unk1: [u8; 4]
	/// Compressed size of the block, or 32,000 to signify the block is not compressed.
	#[br(pad_before = 4)]
	compressed_size: u32,
	/// The decompressed size of the block, or the full size of the data for uncompressed blocks.
	decompressed_size: u32,

	/// Offset within the patch file that the payload starts.
	#[br(map = |value: PosValue<()>| value.pos)]
	offset: u64,
}

impl BlockHeader {
	/// Whether this block is compressed within the patch file.
	pub fn is_compressed(&self) -> bool {
		self.compressed_size != UNCOMPRESSED_MARKER_SIZE
	}

	/// Size of the block payload in the patch file.
	pub fn payload_size(&self) -> u32 {
		match self.is_compressed() {
			true => self.compressed_size,
			false => self.decompressed_size,
		}
	}
}

/// Update the header of a file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct HeaderUpdateCommand {
	/// The kind of file that should be updated.
	file_kind: HeaderFileKind,
	/// The kind of header that should be updated within the file.
	header_kind: HeaderKind,

	/// File to modify.
	#[br(pad_before = 1)]
	file: SqPackFile,

	/// Offset within the patch file that the payload starts.
	#[br(map = |value: PosValue<()>| value.pos)]
	offset: u64,

	// It's _always_ 1kb of data.
	/// Number of bytes that should be written
	#[br(calc = 1024)]
	size: u32,
}

#[allow(missing_docs)]
#[binread]
#[br(repr = u8)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HeaderFileKind {
	Dat = b'D',
	Index = b'I',
}

#[allow(missing_docs)]
#[binread]
#[br(repr = u8)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HeaderKind {
	/// The primary SqPack version header, at offset 0 in all SqPack files.
	Version = b'V',
	/// SqPack .dat header, only sent for `file_kind == Dat`. Starts at offset 1024.
	Data = b'D',
	/// SqPack .index header, only sent for `file_kind == Index`. Starts at offset 1024.
	Index = b'I',
}

/// Update an entry in a SqPack index file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct IndexUpdateCommand {
	/// Kind of modification to make.
	kind: IndexUpdateKind,
	/// If the target entry is a synonym.
	#[br(map = |value: u8| value != 0)]
	is_synonym: bool,
	// align: u8
	/// Index file to modify.
	#[br(pad_before = 1)]
	file: SqPackFile,
	/// Hash key of the index entry to modify.
	file_hash: u64,
	///
	block_offset: u32,
	///
	block_count: u32,
}

#[allow(missing_docs)]
#[binread]
#[br(repr = u8)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IndexUpdateKind {
	Add = b'A',
	Delete = b'D',
}

/// Metadata about the SqPack patch.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct PatchInfoCommand {
	///
	status: u8,
	///
	version: u8,

	// align: u8,
	///
	#[br(pad_before = 1)]
	install_size: u64,
}

/// Metadata about the target SqPack install.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct TargetInfoCommand {
	// unk1: [u8; 3]
	/// The target platform of this patch.
	#[br(pad_before = 3)]
	platform: TargetPlatform,

	/// The target game service region of this patch.
	region: TargetRegion,

	///
	#[br(map = |value: u16| value !=0)]
	is_debug: bool,

	///
	version: u16,

	// TODO: these two seem off, look into what they're supposed to represent.
	#[getset(skip)]
	_deleted_data_size: u64,
	#[getset(skip)]
	_seek_count: u64,
}

#[allow(missing_docs)]
#[binread]
#[br(repr = u16)]
#[derive(Debug, Clone, Copy)]
pub enum TargetPlatform {
	Win32 = 0,
	Ps3 = 1,
	Ps4 = 2,
	Unknown = 3,
}

#[allow(missing_docs)]
#[binread]
#[br(repr = i16)]
#[derive(Debug, Clone, Copy)]
pub enum TargetRegion {
	Global = -1,
	// ZH seems to use global, KR is unknown
}
