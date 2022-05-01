use std::io::{Cursor, SeekFrom};

use binrw::{binread, until_eof, BinRead};

use crate::{
	error::{Error, Result},
	File,
};

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
pub struct ExcelData {
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

impl File for ExcelData {
	fn read(data: Vec<u8>) -> Result<Self> {
		<Self as BinRead>::read(&mut Cursor::new(data))
			.map_err(|error| Error::Resource(error.into()))
	}
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
