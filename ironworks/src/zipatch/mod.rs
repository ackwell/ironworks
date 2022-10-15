use std::{collections::HashMap, fs};

use binrw::{binread, until, BinRead, NullString};

use crate::error::Result;

// TODO: it might not be worth reading all of this into memory at once - maybe remove this top level binread, make the magic check manual, then expose an iterator for the chunks
#[derive(Debug, BinRead)]
#[br(big, magic = b"\x91ZIPATCH\x0D\x0A\x1A\x0A")]
pub struct ZiPatch {
	// TODO: limit
	#[br(parse_with = until(|chunk: &Chunk| matches!(chunk.kind, ChunkKind::EndOfFile)))]
	chunks: Vec<Chunk>,
}

#[derive(Debug, BinRead)]
#[br(big)]
struct Chunk {
	// TODO: use this? do individual chunks own this logic or nah?
	size: u32,
	// note +4 for the header which isn't included in the size for #reasons
	#[br(pad_size_to = size + 4)]
	kind: ChunkKind,
	crc: u32,
}

// TODO: rename, this isn't just a kind
#[derive(Debug, BinRead)]
#[br(big)]
enum ChunkKind {
	#[br(magic = b"FHDR")]
	FileHeader(FileHeader),

	#[br(magic = b"APLY")]
	ApplyOption(ApplyOption),

	#[br(magic = b"SQPK")]
	SqPack(SqPack),

	#[br(magic = b"EOF_")]
	EndOfFile,
}

#[derive(Debug, BinRead)]
#[br(big)]
struct FileHeader {
	// unk1: u16
	#[br(pad_before = 2)]
	version: u8,
	// unk2: u8
	#[br(pad_before = 1)]
	// note don't trust this it doesn't seem to match the file name's suggestion
	patch_kind: PatchKind,
	#[br(pad_before = 24)]
	// unk3: [u8; 24]
	hash: u32,
	// unk4: [u8; 212]
}

#[derive(Debug, BinRead)]
#[br(big)]
enum PatchKind {
	#[br(magic = b"DIFF")]
	Diff,

	#[br(magic = b"HIST")]
	Hist,
}

#[derive(Debug, BinRead)]
#[br(big)]
struct ApplyOption {
	option: Option,
	#[br(pad_before = 4)]
	// unk1: u32,
	value: u32,
	// unk2: [u8; 4],
}

#[derive(Debug, BinRead)]
#[br(big, repr = u32)]
enum Option {
	IgnoreMissing = 1,
	IgnoreMismatch = 2,
}

// TODO: not happy with naming on most of the sqpack stuff
#[derive(Debug, BinRead)]
#[br(big)]
struct SqPack {
	size: u32,
	// operation: u8,
	payload: SqPackPayload,
}

#[derive(Debug, BinRead)]
#[br(big)]
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

#[derive(Debug, BinRead)]
#[br(big)]
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

#[derive(Debug, BinRead)]
#[br(big)]
struct SqPackDelete {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: File,
	offset: u32,
	count: u32,
}

#[derive(Debug, BinRead)]
#[br(big)]
struct SqPackExpand {
	// unk1: [u8; 3]
	#[br(pad_before = 3)]
	file: File,
	offset: u32,
	count: u32,
}

// TODO: put this somewhere more sensible
// TODO: name
#[derive(Debug, BinRead)]
#[br(big)]
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
}

#[derive(Debug, BinRead)]
#[br(repr = u8)]
#[repr(u8)]
enum SqPackFileOperationKind {
	AddFile = b'A',
	// Unused?
	DeleteFile = b'D',
	// Unused?
	MakeDirTree = b'M',
	RemoveAll = b'R',
}

#[derive(Debug, BinRead)]
#[br(big)]
struct SqPackHeaderUpdate {
	file_kind: SqPackHeaderFileKind,
	header_kind: SqPackHeaderHeaderKind,
	path: File,
	#[br(count = 1024)] // not using an array to avoid it inlining all of that into the enum
	payload: Vec<u8>,
}

#[derive(Debug, BinRead)]
#[br(repr = u8)]
#[repr(u8)]
enum SqPackHeaderFileKind {
	Dat = b'D',
	Index = b'I',
}

// TODO: these names jfc
#[derive(Debug, BinRead)]
#[br(repr = u8)]
#[repr(u8)]
enum SqPackHeaderHeaderKind {
	Version = b'V',
	Data = b'D',
	Index = b'I',
}

#[derive(Debug, BinRead)]
#[br(big)]
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

#[derive(Debug, BinRead)]
#[br(repr = u8)]
#[repr(u8)]
enum SqPackIndexUpdateKind {
	Add = b'A',
	Delete = b'D',
}

#[derive(Debug, BinRead)]
#[br(big)]
struct SqPackPatchInfo {
	status: u8,
	version: u8,
	// align: u8,
	#[br(pad_before = 1)]
	install_size: u64,
	// padding?
}

#[derive(Debug, BinRead)]
#[br(big)]
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

#[derive(Debug, BinRead)]
#[br(repr = u16)]
enum Platform {
	Win32 = 0,
	Ps3 = 1,
	Ps4 = 2,
	Unknown = 3,
}

#[derive(Debug, BinRead)]
#[br(repr = i16)]
enum Region {
	Global = -1,
	// ZH seems to use global, KR is unknown
}

pub fn test() -> Result<ZiPatch> {
	let mut file = fs::File::open(
		// "/mnt/c/Users/ackwell/code/xiv/patches/game/4e9a232b/H2017.06.06.0000.0001d.patch",
		"/mnt/c/Users/ackwell/code/xiv/patches/game/4e9a232b/D2022.08.05.0000.0000.patch",
	)?;

	// eep; todo doc this if it works and i end up using it lmao
	// it's about 2x as fast as a bufreader wrapper on a file. worth it?
	let test = unsafe { memmap2::Mmap::map(&file) }.unwrap();
	let mut file = std::io::Cursor::new(test);

	// let mut file = BufReader::new(file);

	let zipatch = ZiPatch::read(&mut file).unwrap();

	let mut counts = HashMap::<String, u32>::new();

	// let test = &zipatch.chunks[1];
	// let test = zipatch.chunks.len();
	// let test = &zipatch.chunks[test - 1];
	for chunk in &zipatch.chunks {
		let foo = match &chunk.kind {
			ChunkKind::SqPack(sqpack) => match &sqpack.payload {
				SqPackPayload::FileOperation(fo) => {
					format!("SQPACK:FileOperation:{:?}:{}", fo.kind, fo.path)
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
			ChunkKind::EndOfFile => "EOF".to_string(),
		};
		counts.entry(foo).and_modify(|v| *v += 1).or_insert(1);
	}

	println!("{counts:#?}");

	Ok(zipatch)
}
