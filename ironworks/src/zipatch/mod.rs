use std::{collections::HashMap, fs};

use binrw::{binread, until, BinRead, NullString};

use crate::error::Result;

// TODO: it might not be worth reading all of this into memory at once - maybe remove this top level binread, make the magic check manual, then expose an iterator for the chunks
#[binread]
#[br(big, magic = b"\x91ZIPATCH\x0D\x0A\x1A\x0A")]
#[derive(Debug)]
pub struct ZiPatch {
	// TODO: limit
	#[br(parse_with = until(|chunk: &Chunk| matches!(chunk.kind, ChunkKind::EndOfFile)))]
	chunks: Vec<Chunk>,
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct Chunk {
	// TODO: use this? do individual chunks own this logic or nah?
	size: u32,
	// note +4 for the header which isn't included in the size for #reasons
	#[br(pad_size_to = size + 4)]
	kind: ChunkKind,
	crc: u32,
}

// TODO: rename, this isn't just a kind
#[binread]
#[br(big)]
#[derive(Debug)]
enum ChunkKind {
	#[br(magic = b"FHDR")]
	FileHeader(FileHeader),

	#[br(magic = b"APLY")]
	ApplyOption(ApplyOption),

	#[br(magic = b"ADIR")]
	AddDirectory(AddDirectory),

	#[br(magic = b"DELD")]
	DeleteDirectory(DeleteDirectory),

	#[br(magic = b"SQPK")]
	SqPack(SqPack),

	#[br(magic = b"EOF_")]
	EndOfFile,
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct FileHeader {
	// unk1: u16
	#[br(pad_before = 2)]
	version: u8,
	// unk2: u8
	// note don't trust this it doesn't seem to match the file name's suggestion
	#[br(pad_before = 1)]
	patch_kind: PatchKind,
	entry_files: u32,

	#[br(if(version == 3))]
	v3: Option<FileHeaderV3>,
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct FileHeaderV3 {
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
#[derive(Debug)]
enum PatchKind {
	#[br(magic = b"DIFF")]
	Diff,

	#[br(magic = b"HIST")]
	Hist,
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct ApplyOption {
	option: OptionKind,
	#[br(pad_before = 4)]
	// unk1: u32,
	value: u32,
	// unk2: [u8; 4],
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug)]
enum OptionKind {
	IgnoreMissing = 1,
	IgnoreMismatch = 2,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct AddDirectory {
	#[br(temp)]
	length: u32,
	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct DeleteDirectory {
	#[br(temp)]
	length: u32,
	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

// TODO: not happy with naming on most of the sqpack stuff
#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPack {
	size: u32,
	// operation: u8,
	payload: SqPackPayload,
}

#[binread]
#[br(big)]
#[derive(Debug)]
enum SqPackPayload {
	#[br(magic = b"A")]
	Add(SqPackAdd),

	#[br(magic = b"D")]
	Delete(SqPackDelete),

	#[br(magic = b"E")]
	Expand(SqPackExpand),

	#[br(magic = b"F")]
	FileOperation(SqPackFileOperation),

	#[br(magic = b"H")]
	HeaderUpdate(SqPackHeaderUpdate),

	// Unused?
	#[br(magic = b"I")]
	IndexUpdate(SqPackIndexUpdate),

	// Unused?
	#[br(magic = b"X")]
	PatchInfo(SqPackPatchInfo),

	#[br(magic = b"T")]
	TargetInfo(SqPackTargetInfo),
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackAdd {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: File,
	offset: u32,
	count: u32,
	delete_count: u32,
	// TODO:
	// data - store the full reader offset for this point maybe?
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackDelete {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: File,
	offset: u32,
	count: u32,
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackExpand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: File,
	offset: u32,
	count: u32,
}

// TODO: put this somewhere more sensible
// TODO: name
#[binread]
#[br(big)]
#[derive(Debug)]
struct File {
	main_id: u16,
	sub_id: u16,
	file_id: u32,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct SqPackFileOperation {
	kind: SqPackFileOperationKind,
	#[br(pad_before = 2)]
	// unk1: [u8; 2]
	offset: u64,
	size: u64,
	#[br(temp)]
	path_length: u32,
	#[br(pad_after = 2)]
	// todo: repository id?
	expansion_id: u16,
	// unk2: [u8; 2]

	// ??????
	#[br(pad_size_to = path_length)]
	path: NullString,
	// data here i assume? looks like there's a whole other structure going on
	// check https://github.com/goatcorp/FFXIVQuickLauncher/blob/master/src/XIVLauncher.Common/Patching/ZiPatch/Chunk/SqpkCommand/SqpkFile.cs#L51
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum SqPackFileOperationKind {
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
struct SqPackHeaderUpdate {
	file_kind: SqPackHeaderFileKind,
	header_kind: SqPackHeaderHeaderKind,
	path: File,
	#[br(count = 1024)] // not using an array to avoid it inlining all of that into the enum
	payload: Vec<u8>,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum SqPackHeaderFileKind {
	Dat = b'D',
	Index = b'I',
}

// TODO: these names jfc
#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum SqPackHeaderHeaderKind {
	Version = b'V',
	Data = b'D',
	Index = b'I',
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackIndexUpdate {
	kind: SqPackIndexUpdateKind,
	is_synonym: u8, //bool
	// align: u8
	#[br(pad_before = 1)]
	index: File,
	file_hash: u64,
	block_offset: u32,
	block_number: u32,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
#[repr(u8)]
enum SqPackIndexUpdateKind {
	Add = b'A',
	Delete = b'D',
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackPatchInfo {
	status: u8,
	version: u8,
	// align: u8,
	#[br(pad_before = 1)]
	install_size: u64,
	// padding?
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct SqPackTargetInfo {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	platform: Platform,
	region: Region,
	is_debug: u16, // bool
	version: u16,
	// TODO: these two seem off, probably shouldn't expose
	deleted_data_size: u64,
	seek_count: u64,
	//padding
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
enum Platform {
	Win32 = 0,
	Ps3 = 1,
	Ps4 = 2,
	Unknown = 3,
}

#[binread]
#[br(repr = i16)]
#[derive(Debug)]
enum Region {
	Global = -1,
	// ZH seems to use global, KR is unknown
}

pub fn test() -> Result<ZiPatch> {
	let mut file = fs::File::open(
		// "/mnt/c/Users/ackwell/code/xiv/patches/game/4e9a232b/H2017.06.06.0000.0001d.patch",
		"/mnt/c/Users/ackwell/code/xiv/patches/game/4e9a232b/D2022.08.05.0001.0000.patch",
		// "/mnt/c/Users/ackwell/code/xiv/patches/boot/2b5cbc63/D2022.08.05.0000.0001.patch",
		// "/mnt/c/Users/ackwell/code/xiv/patches/game/ex4/1bf99b87/D2022.09.30.0000.0000.patch",
	)?;

	// eep; todo doc this if it works and i end up using it lmao
	// it's about 2x as fast as a bufreader wrapper on a file. worth it?
	let test = unsafe { memmap2::Mmap::map(&file) }.unwrap();
	let mut file = std::io::Cursor::new(test);

	// let mut file = BufReader::new(file);

	let zipatch = ZiPatch::read(&mut file).unwrap();

	println!("{zipatch:#?}");

	let mut counts = HashMap::<String, u32>::new();

	// let test = &zipatch.chunks[1];
	// let test = zipatch.chunks.len();
	// let test = &zipatch.chunks[test - 1];
	for chunk in &zipatch.chunks {
		let foo = match &chunk.kind {
			ChunkKind::SqPack(sqpack) => match &sqpack.payload {
				SqPackPayload::FileOperation(fo) => {
					format!(
						"SQPACK:FileOperation:{:?}:{} ({}, {})",
						fo.kind, fo.path, fo.offset, fo.size
					)
				}
				SqPackPayload::IndexUpdate(fo) => {
					format!("SQPACK:IndexUpdate:{:?}", fo.kind)
				}
				SqPackPayload::Add(_) => "SQPACK:Add".to_string(),
				SqPackPayload::HeaderUpdate(_) => "SQPACK:HeaderUpdate".to_string(),
				other => format!("SQPACK:{other:?}"),
			},
			ChunkKind::ApplyOption(_) => "APPLY".to_string(),
			ChunkKind::FileHeader(_) => "FHEAD".to_string(),
			ChunkKind::AddDirectory(_) => "ADIR".to_string(),
			ChunkKind::DeleteDirectory(_) => "DELD".to_string(),
			ChunkKind::EndOfFile => "EOF".to_string(),
		};
		counts.entry(foo).and_modify(|v| *v += 1).or_insert(1);
	}

	println!("{counts:#?}");

	Ok(zipatch)
}
