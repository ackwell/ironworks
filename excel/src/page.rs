use std::io::Cursor;

use binrw::{binread, BinRead};

use crate::error::{Error, Result};

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
pub struct PageHeader {
	#[allow(dead_code)]
	version: u16,
	// unknown1: u16,
	#[br(pad_before = 2, temp)]
	index_size: u32,
	// unknown2: [u16; 10],
	#[br(pad_before = 20, count = index_size / RowDefinition::SIZE as u32)]
	pub rows: Vec<RowDefinition>,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct RowDefinition {
	pub row_id: u32,
	pub offset: u32,
}

impl RowDefinition {
	const SIZE: usize = 8;
}

#[derive(Debug)]
pub struct Page {
	pub header: PageHeader,
	pub data: Vec<u8>,
}

impl Page {
	pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
		let mut cursor = Cursor::new(&bytes);
		let header = PageHeader::read(&mut cursor).map_err(|error| {
			Error::InvalidResource(format!("Failed to read ExcelPage header: {}", error))
		})?;

		Ok(Self {
			header,
			data: bytes,
		})
	}
}
