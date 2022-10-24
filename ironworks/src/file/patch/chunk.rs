use std::sync::{Arc, Mutex};

use binrw::{binread, meta::ReadEndian, BinRead};

use crate::{error::Result, FileStream};

use super::lazy::LazyStreamReader;

#[derive(Debug)]
pub enum Chunk {
	FileHeader,

	Apply(LazyStreamReader<ApplyOption>),

	AddDirectory,

	DeleteDirectory,

	SqPack,

	EndOfFile,
}

impl Chunk {
	pub(super) fn read(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Self {
		// Get the magic for this chunk.
		let mut handle = stream.lock().unwrap();
		let magic = <[u8; 4]>::read(&mut *handle).expect("e");
		drop(handle);

		match &magic {
			b"FHDR" => Self::FileHeader,
			b"APLY" => Self::Apply(LazyStreamReader::new(stream)),
			b"ADIR" => Self::AddDirectory,
			b"DELD" => Self::DeleteDirectory,
			b"SQPK" => Self::SqPack,
			b"EOF_" => Self::EndOfFile,
			// temp obv
			other => todo!("Unknown chunk kind {other:?}"),
		}
	}
}

fn eager<T: BinRead<Args = ()> + ReadEndian>(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Result<T> {
	let mut handle = stream.lock().unwrap();
	Ok(T::read(&mut *handle)?)
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct ApplyOption {
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
