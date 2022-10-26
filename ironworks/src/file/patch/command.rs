use binrw::{binread, NullString, PosValue};

// todo: doc this.
// dat`"{main_id:02x}{sub_id:04x}.{platform}.dat{file_id}"`
// idx`"{main_id:02x}{sub_id:04x}.{platform}.index{file_id>0?file_id:""}"`
#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackFile {
	main_id: u16,
	sub_id: u16,
	file_id: u32,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct AddCommand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: SqPackFile,
	// target file offset to start writing, in bytes
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
	// size of data to copy, in bytes
	#[br(map = |value: u32| value << 7)]
	data_size: u32,
	// no. of bytes to blank after write op
	#[br(map = |value: u32| value << 7)]
	delete_size: u32,

	// offset in bytes within the zipatch itself to read the data
	#[br(map = |v: PosValue<()>| v.pos)]
	source_offset: u64,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct DeleteCommand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: SqPackFile,
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
	#[br(map = |value: u32| value << 7)]
	delete_size: u32,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct ExpandCommand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: SqPackFile,
	#[br(map = |value: u32| value << 7)]
	target_offset: u32,
	#[br(map = |value: u32| value << 7)]
	delete_size: u32,
}

#[binread]
#[derive(Debug)]
pub struct FileOperationCommand {
	kind: FileOperationKind,
	// unk1: [u8; 2]
	#[br(pad_before = 2)]
	offset: u64,
	size: u64,
	#[br(temp)]
	path_length: u32,
	#[br(pad_after = 2)]
	// todo: repository id?
	expansion_id: u16,
	// unk2: [u8; 2]
	#[br(pad_size_to = path_length)]
	path: NullString,
	// data here i assume? looks like there's a whole other structure going on
	// check https://github.com/goatcorp/FFXIVQuickLauncher/blob/master/src/XIVLauncher.Common/Patching/ZiPatch/Chunk/SqpkCommand/SqpkFile.cs#L51
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum FileOperationKind {
	AddFile = b'A',
	// Unused?
	DeleteFile = b'D',
	// Unused?
	MakeDirTree = b'M',
	RemoveAll = b'R',
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct HeaderUpdateCommand {
	file_kind: HeaderFileKind,
	header_kind: HeaderKind,
	path: SqPackFile,
	// TODO: handle this the same way as the add command
	#[br(count = 1024)] // not using an array to avoid it inlining all of that into the enum
	payload: Vec<u8>,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum HeaderFileKind {
	Dat = b'D',
	Index = b'I',
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum HeaderKind {
	Version = b'V',
	Data = b'D',
	Index = b'I',
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct IndexUpdateCommand {
	kind: IndexUpdateKind,
	#[br(map = |value: u8| value != 0)]
	is_synonym: bool,
	// align: u8
	#[br(pad_before = 1)]
	index: SqPackFile,
	file_hash: u64,
	block_offset: u32,
	block_number: u32,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum IndexUpdateKind {
	Add = b'A',
	Delete = b'D',
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct PatchInfoCommand {
	status: u8,
	version: u8,
	// align: u8,
	#[br(pad_before = 1)]
	install_size: u64,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct TargetInfoCommand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	platform: TargetPlatform,
	region: TargetRegion,
	#[br(map = |value: u16| value !=0)]
	is_debug: bool,
	version: u16,
	// TODO: these two seem off, probably shouldn't expose
	deleted_data_size: u64,
	seek_count: u64,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
enum TargetPlatform {
	Win32 = 0,
	Ps3 = 1,
	Ps4 = 2,
	Unknown = 3,
}

#[binread]
#[br(repr = i16)]
#[derive(Debug)]
enum TargetRegion {
	Global = -1,
	// ZH seems to use global, KR is unknown
}
