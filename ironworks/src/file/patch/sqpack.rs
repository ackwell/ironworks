use std::io::{Read, Seek};

use binrw::{binread, BinRead, BinResult, NullString, PosValue, ReadOptions};

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
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		// TODO: should I use the size?
		let _size = u32::read_options(reader, options, ())?;
		let pos = reader.stream_position()?;
		let magic = u8::read_options(reader, options, ())?;

		let command = match magic {
			b'A' => Self::Add(AddCommand::read_options(reader, options, ())?),
			b'D' => Self::Delete(DeleteCommand::read_options(reader, options, ())?),
			b'E' => Self::Expand(ExpandCommand::read_options(reader, options, ())?),
			b'F' => Self::FileOperation(FileOperationCommand::read_options(reader, options, ())?),
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
	offset: u32,
	count: u32,
	delete_count: u32,
	// TODO:
	// data - store the full reader offset for this point maybe?
	#[br(map = |v: PosValue<()>| v.pos)]
	test: u64,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct DeleteCommand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: SqPackFile,
	offset: u32,
	count: u32,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct ExpandCommand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: SqPackFile,
	offset: u32,
	count: u32,
}

#[binread]
#[derive(Debug)]
#[br(big)]
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
