use std::io::{Read, Seek};

use binrw::{binread, BinRead, BinResult, PosValue, ReadOptions};

#[derive(Debug)]
pub enum SqPackChunk {
	Add(AddCommand),
	Delete,
	Expand,
	FileOperation,
	HeaderUpdate,
	IndexUpdate,
	PatchInfo,
	TargetInfo,
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
			b'D' => Self::Delete,
			b'E' => Self::Expand,
			b'F' => Self::FileOperation,
			b'H' => Self::HeaderUpdate,
			b'I' => Self::IndexUpdate,
			b'X' => Self::PatchInfo,
			b'T' => Self::TargetInfo,
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
