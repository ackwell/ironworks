use std::sync::{Arc, Mutex};

use binrw::{binread, meta::ReadEndian, BinRead};
use getset::{CopyGetters, Getters};

use crate::{
	error::{Error, ErrorValue, Result},
	FileStream,
};

use super::{lazy::LazyStreamReader, sqpack::SqPackChunk};

#[derive(Debug)]
pub enum Chunk {
	FileHeader(LazyStreamReader<FileHeaderChunk>),
	Apply(ApplyChunk),
	AddDirectory(AddDirectoryChunk),
	DeleteDirectory(DeleteDirectoryChunk),
	SqPack(SqPackChunk),
	EndOfFile,
}

impl Chunk {
	pub(super) fn read(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Result<Self> {
		// Get the magic for this chunk.
		let mut handle = stream.lock().unwrap();
		let magic = <[u8; 4]>::read(&mut *handle)?;
		drop(handle);

		let chunk = match &magic {
			b"FHDR" => Self::FileHeader(LazyStreamReader::new(stream)),
			b"APLY" => Self::Apply(eager(stream)?),
			b"ADIR" => Self::AddDirectory(eager(stream)?),
			b"DELD" => Self::DeleteDirectory(eager(stream)?),
			b"SQPK" => Self::SqPack(SqPackChunk::read(stream)?),
			b"EOF_" => Self::EndOfFile,
			other => {
				return Err(Error::Invalid(
					ErrorValue::Other("chunk magic".into()),
					format!("unknown chunk magic {other:?}"),
				))
			}
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
#[derive(Debug, Getters, CopyGetters)]
pub struct FileHeaderChunk {
	// unk1: u16
	#[br(temp)]
	#[br(pad_before = 2)]
	version: u8,

	// unk2: u8
	// note don't trust this it doesn't seem to match the file name's suggestion
	#[br(pad_before = 1)]
	#[get_copy = "pub"]
	patch_kind: PatchKind,

	#[get_copy = "pub"]
	entry_files: u32,

	#[br(if(version == 3))]
	#[get = "pub"]
	v3: Option<FileHeaderV3>,
}

#[binread]
#[br(big)]
#[derive(Debug, Clone, Copy)]
pub enum PatchKind {
	#[br(magic = b"DIFF")]
	Diff,

	#[br(magic = b"HIST")]
	Hist,
}

#[binread]
#[br(big)]
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct FileHeaderV3 {
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
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct ApplyChunk {
	option: OptionKind,

	#[br(pad_before = 4)]
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
#[br(big)]
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct AddDirectoryChunk {
	#[br(temp)]
	length: u32,

	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}

#[binread]
#[br(big)]
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct DeleteDirectoryChunk {
	#[br(temp)]
	length: u32,

	#[br(
		count = length,
		try_map = String::from_utf8,
	)]
	path: String,
}
