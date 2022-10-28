use std::io::{Read, Seek, SeekFrom};

use binrw::{binread, BinRead, BinResult, NullString, PosValue, ReadOptions};
use getset::{CopyGetters, Getters};

const UNCOMPRESSED_MARKER_SIZE: u32 = 32_000;

// todo: doc this.
// dat`"{main_id:02x}{sub_id:04x}.{platform}.dat{file_id}"`
// idx`"{main_id:02x}{sub_id:04x}.{platform}.index{file_id>0?file_id:""}"`
#[binread]
#[br(big)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct SqPackFile {
	main_id: u16,
	sub_id: u16,
	file_id: u32,
}

/// Write data to a file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct AddCommand {
	// unk1: [u8; 3]
	/// File to write to.
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
	#[br(pad_before = 3)]
	file: SqPackFile,
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
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
	#[br(pad_before = 3)]
	file: SqPackFile,
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
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
	#[br(pad_before = 2)]
	#[get_copy = "pub"]
	target_offset: u64,

	#[get_copy = "pub"]
	target_size: u64,

	#[br(temp)]
	path_length: u32,

	// todo: repository id?
	#[get_copy = "pub"]
	expansion_id: u16,

	// unk2: [u8; 2]
	#[br(pad_before = 2)]
	#[br(pad_size_to = path_length)]
	#[get = "pub"]
	path: NullString,

	#[br(args(operation_magic, command_start, command_size))]
	#[get = "pub"]
	operation: FileOperation,
}

#[binread]
#[br(import(magic: u8, command_start: u64, command_size: u32))]
#[derive(Debug)]
pub enum FileOperation {
	#[br(pre_assert(magic == b'A'))]
	AddFile(
		#[br(parse_with = parse_block_headers)]
		#[br(args(command_start, command_size))]
		Vec<BlockHeader>,
	),

	// Unused?
	#[br(pre_assert(magic == b'D'))]
	DeleteFile,

	// Unused?
	#[br(pre_assert(magic == b'M'))]
	MakeDirTree,

	#[br(pre_assert(magic == b'R'))]
	RemoveAll,
}

fn parse_block_headers<R: Read + Seek>(
	reader: &mut R,
	options: &ReadOptions,
	(command_start, command_size): (u64, u32),
) -> BinResult<Vec<BlockHeader>> {
	let command_end = u64::from(command_size) + command_start;

	let mut headers = vec![];

	// Read headers while there's space left in the command's data.
	while reader.stream_position()? < command_end {
		let header = BlockHeader::read_options(reader, options, ())?;

		// Blocks can be compressed or uncompressed in source data, and are sorta-kinda aligned in typical Square fashion.
		let source_size = match header.compressed_size != UNCOMPRESSED_MARKER_SIZE {
			true => header.compressed_size,
			false => header.decompressed_size,
		};
		let aligned_size = (u64::from(source_size) + 0x8F) & 0xFFFFFF80;

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

// This is identical to the `BlockHeader` in `sqpack::file` - look into sharing.
/// Block of potentially-compressed data
#[binread]
#[br(little)] // REALLY?
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct BlockHeader {
	// Practically always 16.
	header_size: u32,
	// unk1: [u8; 4]
	#[br(pad_before = 4)]
	compressed_size: u32,
	decompressed_size: u32,

	#[br(map = |value: PosValue<()>| value.pos)]
	offset: u64,
}

/// Update the header of a file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct HeaderUpdateCommand {
	file_kind: HeaderFileKind,
	header_kind: HeaderKind,

	#[br(pad_before = 1)]
	file: SqPackFile,

	#[br(map = |value: PosValue<()>| value.pos)]
	offset: u64,

	// It's _always_ 1kb of data.
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
	Version = b'V',
	Data = b'D',
	Index = b'I',
}

/// Update an entry in a SqPack index file.
#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct IndexUpdateCommand {
	kind: IndexUpdateKind,
	#[br(map = |value: u8| value != 0)]
	is_synonym: bool,
	// align: u8
	#[br(pad_before = 1)]
	file: SqPackFile,
	file_hash: u64,
	block_offset: u32,
	block_number: u32,
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
