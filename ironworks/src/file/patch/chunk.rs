use std::sync::{Arc, Mutex};

use binrw::{binread, meta::ReadEndian, BinRead};
use getset::{CopyGetters, Getters};

use crate::{error::Result, FileStream};

#[derive(Debug)]
pub enum Chunk {
	FileHeader,
	Apply(ApplyChunk),
	AddDirectory(AddDirectoryChunk),
	DeleteDirectory(DeleteDirectoryChunk),
	SqPack,
	EndOfFile,
}

impl Chunk {
	pub(super) fn read(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Result<Self> {
		// Get the magic for this chunk.
		let mut handle = stream.lock().unwrap();
		let magic = <[u8; 4]>::read(&mut *handle)?;
		drop(handle);

		let chunk = match &magic {
			b"FHDR" => Self::FileHeader,
			b"APLY" => Self::Apply(eager(stream)?),
			b"ADIR" => Self::AddDirectory(eager(stream)?),
			b"DELD" => Self::DeleteDirectory(eager(stream)?),
			b"SQPK" => Self::SqPack,
			b"EOF_" => Self::EndOfFile,
			// temp obv
			other => todo!("Unknown chunk kind {other:?}"),
		};

		Ok(chunk)
	}
}

fn eager<T: BinRead<Args = ()> + ReadEndian>(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Result<T> {
	let mut handle = stream.lock().unwrap();
	Ok(T::read(&mut *handle)?)
}

#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
pub struct ApplyChunk {
	#[get_copy = "pub"]
	option: OptionKind,

	#[br(pad_before = 4)]
	#[get_copy = "pub"]
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
#[derive(Debug, Getters)]
#[br(big)]
pub struct AddDirectoryChunk {
	#[br(temp)]
	length: u32,

	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	#[get = "pub"]
	path: String,
}

#[binread]
#[derive(Debug, Getters)]
#[br(big)]
pub struct DeleteDirectoryChunk {
	#[br(temp)]
	length: u32,

	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	#[get = "pub"]
	path: String,
}
