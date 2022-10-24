use std::sync::{Arc, Mutex};

use binrw::{binread, meta::ReadEndian, BinRead, PosValue};

use crate::{
	error::{Error, ErrorValue, Result},
	FileStream,
};

use super::lazy::LazyStreamReader;

#[derive(Debug)]
pub enum SqPackChunk {
	// This being lazy is an immense time save - but does it _realistically_ matter? are consumers ever going to iterate and _not_ want adds?
	Add(LazyStreamReader<AddCommand>),
	Delete,
	Expand,
	FileOperation,
	HeaderUpdate,
	IndexUpdate,
	PatchInfo,
	TargetInfo,
}

impl SqPackChunk {
	pub(super) fn read(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Result<Self> {
		let mut handle = stream.lock().unwrap();
		let size = u32::read_be(&mut *handle)?;
		let magic = u8::read(&mut *handle)?;
		drop(handle);

		let command = match magic {
			b'A' => Self::Add(LazyStreamReader::new(stream)),
			b'D' => Self::Delete,
			b'E' => Self::Expand,
			b'F' => Self::FileOperation,
			b'H' => Self::HeaderUpdate,
			b'I' => Self::IndexUpdate,
			b'X' => Self::PatchInfo,
			b'T' => Self::TargetInfo,
			other => {
				return Err(Error::Invalid(
					ErrorValue::Other("sqpack command magic".into()),
					format!("unknown command {other:?}"),
				))
			}
		};

		Ok(command)
	}
}

// todo this is duped, nuke
fn eager<T: BinRead<Args = ()> + ReadEndian>(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Result<T> {
	let mut handle = stream.lock().unwrap();
	Ok(T::read(&mut *handle)?)
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
