use std::io::Cursor;

use binrw::{binread, BinRead};

use crate::error::{Error, Result};

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXHF")]
pub struct ExcelHeader {
	version: u16,
	pub row_size: u16,
	#[br(temp)]
	column_count: u16,
	#[br(temp)]
	page_count: u16,
	#[br(temp)]
	language_count: u16,
	// unknown1: u16,
	// unknown2: u8,
	#[br(pad_before = 3)]
	kind: ExcelSheetKind,
	// unknown3: u16,
	#[br(pad_before = 2)]
	row_count: u32,
	// unknown4: [u32; 2]
	#[br(pad_before = 8, count = column_count)]
	pub columns: Vec<ExcelColumnDefinition>,

	#[br(count = page_count)]
	pub pages: Vec<ExcelPageDefinition>,

	#[br(count = language_count)]
	languages: Vec<ExcelLanguageDefinition>,
}

impl ExcelHeader {
	pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
		let header = Self::read(&mut Cursor::new(bytes)).map_err(|error| {
			Error::InvalidResource(format!("Failed to read ExcelHeader: {}", error))
		})?;
		Ok(header)
	}
}

#[derive(BinRead, Debug)]
#[br(big, repr = u8)]
enum ExcelSheetKind {
	Unknown = 0,
	Default = 1,
	Subrows = 2,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct ExcelColumnDefinition {
	pub kind: ExcelColumnKind,
	pub offset: u16,
}

#[derive(BinRead, Debug)]
#[br(big, repr = u16)]
pub enum ExcelColumnKind {
	String = 0x0,
	Bool = 0x1,
	Int8 = 0x2,
	UInt8 = 0x3,
	Int16 = 0x4,
	UInt16 = 0x5,
	Int32 = 0x6,
	UInt32 = 0x7,
	// Unknown = 0x8,
	Float32 = 0x9,
	Int64 = 0xA,
	UInt64 = 0xB,
	// Unknown2 = 0xC,

	// Read as <0>&0b1, <1>&0b10, <2>&0b100, &c
	PackedBool0 = 0x19,
	PackedBool1 = 0x1A,
	PackedBool2 = 0x1B,
	PackedBool3 = 0x1C,
	PackedBool4 = 0x1D,
	PackedBool5 = 0x1E,
	PackedBool6 = 0x1F,
	PackedBool7 = 0x20,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct ExcelPageDefinition {
	pub start_id: u32,
	pub row_count: u32,
}

#[derive(BinRead, Debug)]
#[br(big)]
struct ExcelLanguageDefinition {
	#[br(pad_after = 1)]
	// TODO: this is an enum - but the values are probably usecase-specific. work out how to handle, maybe a-la cat/repo in sqpack?
	language: u8,
	// unkown1: u8, // probably padding
}
