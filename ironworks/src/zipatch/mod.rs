use std::fs;

use binrw::{binread, until, BinRead, NullString};

use crate::error::Result;

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
	Add(Todo),

	#[br(magic = b"D")]
	Delete(Todo),

	#[br(magic = b"E")]
	Expand(Todo),

	#[br(magic = b"F")]
	FileOperation(SqPackFileOperation),

	#[br(magic = b"H")]
	Header(Todo),

	#[br(magic = b"I")]
	Index(Todo),

	#[br(magic = b"X")]
	PatchInfo(Todo),

	#[br(magic = b"T")]
	TargetInfo(Todo),
}

#[derive(Debug, BinRead)]
struct Todo();

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
enum SqPackFileOperationKind {
	AddFile = 'A' as isize,
	// Unused?
	DeleteFile = 'D' as isize,
	// Unused?
	MakeDirTree = 'M' as isize,
	RemoveAll = 'R' as isize,
}

pub fn test() -> Result<ZiPatch> {
	let mut file = fs::File::open(
		"/mnt/c/Users/ackwell/code/xiv/patches/game/4e9a232b/H2017.06.06.0000.0001d.patch",
	)?;

	let zipatch = ZiPatch::read(&mut file).unwrap();

	let test = &zipatch.chunks[200];
	println!("{test:#?}");

	Ok(zipatch)
}
