use std::io::Cursor;

use binrw::{binread, BinRead};

use crate::error::Error;

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXHF")]
struct ExcelHeaderDefinition {
	version: u16,
	row_size: u16,
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
	#[br(count = column_count, pad_before = 8)]
	columns: Vec<ExcelColumnDefinition>,

	#[br(count = page_count)]
	pages: Vec<ExcelPageDefinition>,

	#[br(count = language_count)]
	languages: Vec<ExcelLanguageDefinition>,
}

#[derive(BinRead, Debug)]
#[br(big, repr=u8)]
enum ExcelSheetKind {
	Unknown = 0,
	Default = 1,
	Subrows = 2,
}

#[derive(BinRead, Debug)]
#[br(big)]
struct ExcelColumnDefinition {
	kind: ExcelColumnKind,
	offset: u16,
}

#[derive(BinRead, Debug)]
#[br(big, repr = u16)]
enum ExcelColumnKind {
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
struct ExcelPageDefinition {
	start_id: u32,
	row_count: u32,
}

#[derive(BinRead, Debug)]
#[br(big)]
struct ExcelLanguageDefinition {
	#[br(pad_after = 1)]
	// TODO: this is an enum - but the values are probably usecase-specific. work out how to handle, maybe a-la cat/repo in sqpack?
	language: u8,
	// unkown1: u8, // probably padding
}

#[derive(Debug)]
pub struct ExcelHeader {
	// TODO: don't actually do this - move the shit we need and that's it
	remove_me: ExcelHeaderDefinition,
}

impl ExcelHeader {
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
		let header = ExcelHeaderDefinition::read(&mut Cursor::new(bytes)).unwrap();

		Ok(Self { remove_me: header })
	}
}
