use std::io::Cursor;

use binrw::{binread, BinRead};

use crate::error::{Error, Result};

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
	const SIZE: usize = 8;
}

#[derive(Debug)]
pub struct ExcelPage {
	header: ExcelPageHeader,
	data: Vec<u8>,
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
