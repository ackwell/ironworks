use std::io::Cursor;

use binrw::{binread, BinRead};

use crate::error::{Error, Result};

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
pub struct ExcelPageHeader {
	version: u16,
	// unknown1: u16,
	#[br(pad_before = 2, temp)]
	index_size: u32,
	// unknown2: [u16; 10],
	#[br(pad_before = 20, count = index_size / ExcelRowDefinition::SIZE as u32)]
	pub rows: Vec<ExcelRowDefinition>,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct ExcelRowDefinition {
	pub row_id: u32,
	pub offset: u32,
}

impl ExcelRowDefinition {
	const SIZE: usize = 8;
}

#[derive(Debug)]
pub struct ExcelPage {
	pub header: ExcelPageHeader,
	pub data: Vec<u8>,
}

impl ExcelPage {
	pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
		let mut cursor = Cursor::new(&bytes);
		let header = ExcelPageHeader::read(&mut cursor).map_err(|error| {
			Error::InvalidResource(format!("Failed to read ExcelPage header: {}", error))
		})?;

		Ok(Self {
			header,
			data: bytes,
		})
	}
}
