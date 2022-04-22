use std::io::SeekFrom;

use binrw::{binread, until_eof};

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
pub struct Page {
	_version: u16,
	// unknown1: u16,
	#[br(pad_before = 2, temp)]
	index_size: u32,
	// unknown2: [u16; 10],
	#[br(
    pad_before = 20,
    count = index_size / RowDefinition::SIZE,
  )]
	pub rows: Vec<RowDefinition>,

	// Row offsets are relative to the start of the file - read in the full file as a buffer.
	#[br(
    seek_before = SeekFrom::Start(0),
    parse_with = until_eof,
  )]
	pub data: Vec<u8>,
}

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct RowDefinition {
	pub id: u32,
	pub offset: u32,
}

impl RowDefinition {
	const SIZE: u32 = 8;
}
