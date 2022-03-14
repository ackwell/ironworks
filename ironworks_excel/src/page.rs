use std::io::Cursor;

use binrw::{binread, BinRead};

use crate::error::Result;

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
struct ExcelPageHeader {
	version: u16,
	// unknown1: u16,
	#[br(pad_before = 2, temp)]
	index_size: u32,
	// unknown2: [u16; 10],
	#[br(pad_before = 20, count = index_size / ExcelRowDefinition::SIZE as u32)]
	rows: Vec<ExcelRowDefinition>,
}

#[derive(BinRead, Debug)]
#[br(big)]
struct ExcelRowDefinition {
	row_id: u32,
	offset: u32,
}

impl ExcelRowDefinition {
	pub const SIZE: usize = 8;
}

#[derive(Debug)]
pub struct ExcelPage {
	header: ExcelPageHeader,
}

impl ExcelPage {
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		// TODO error handling
		let mut cursor = Cursor::new(bytes);
		let header = ExcelPageHeader::read(&mut cursor).unwrap();

		Ok(Self { header })
	}
}
