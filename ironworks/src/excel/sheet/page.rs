use std::io::SeekFrom;

use binrw::{binread, until_eof};

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
pub struct Page {
	version: u16,
	// unknown1: u16,
	#[br(pad_before = 2, temp)]
	index_size: u32,
	// unknown2: [u16; 10],
	#[br(
    pad_before = 20,
    count = index_size / RowDefinition::SIZE,
  )]
	rows: Vec<RowDefinition>,

	// Row offsets are relative to the start of the file - read in the full file as a buffer.
	// TODO: Maybe better to work out current offset and save it as something to
	// adjust offsets by?
	#[br(
    seek_before = SeekFrom::Start(0),
    parse_with = until_eof,
  )]
	data: Vec<u8>,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct RowDefinition {
	row_id: u32,
	offset: u32,
}

impl RowDefinition {
	const SIZE: u32 = 8;
}
