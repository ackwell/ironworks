use std::{
	io::SeekFrom,
	ops::Deref,
	sync::{Arc, Mutex},
};

use binrw::{binread, meta::ReadEndian, BinRead};
use derivative::Derivative;
use lazy_init::Lazy;

use crate::{error::Result, FileStream};

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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct LazyStreamReader<T> {
	offset: u64,
	value: Lazy<T>,
	#[derivative(Debug = "ignore")]
	stream: Arc<Mutex<Box<dyn FileStream>>>,
}

impl<T> LazyStreamReader<T> {
	fn new(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Self {
		let mut handle = stream.lock().expect("e");
		let offset = handle.stream_position().expect("e");
		drop(handle);

		Self {
			offset,
			value: Default::default(),
			stream,
		}
	}
}

impl<T: BinRead<Args = ()> + ReadEndian> Deref for LazyStreamReader<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value.get_or_create(|| {
			let mut handle = self.stream.lock().expect("e");
			handle.seek(SeekFrom::Start(self.offset)).expect("e");
			T::read(&mut *handle).expect("e")
		})
	}
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
