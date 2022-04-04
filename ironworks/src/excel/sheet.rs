use std::collections::HashSet;

use binrw::{binread, BinRead};

use crate::error::Result;

use super::resource::Resource;

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet<'r, R> {
	sheet: String,

	resource: &'r R,
}

impl<'r, R: Resource> Sheet<'r, R> {
	pub(crate) fn new(sheet: String, resource: &'r R) -> Self {
		Self { sheet, resource }
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<()> {
		self.subrow(row_id, 0)
	}

	// TODO: u16?
	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, _row_id: u32, _subrow_id: u16) -> Result<()> {
		let mut header_reader = self.resource.header(&self.sheet)?;
		let header = Header::read(&mut header_reader);

		println!("header: {header:#?}");

		Ok(())
	}
}

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXHF")]
struct Header {
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
	kind: SheetKind,
	// unknown3: u16,
	#[br(pad_before = 2)]
	row_count: u32,
	// unknown4: [u32; 2],
	#[br(
		pad_before = 8,
		count = column_count,
	)]
	columns: Vec<ColumnDefinition>,
	#[br(count = page_count)]
	pages: Vec<PageDefinition>,
	// TODO: set
	#[br(
		count = language_count,
		map = LanguageDefinition::to_set,
	)]
	languages: HashSet<u8>,
}

#[binread]
#[derive(Debug)]
#[br(repr = u8)]
enum SheetKind {
	Unknown = 0,
	Default = 1,
	Subrows = 2,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct ColumnDefinition {
	kind: ColumnKind,
	offset: u16,
}

#[binread]
#[derive(Debug)]
#[br(big, repr = u16)]
enum ColumnKind {
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

#[binread]
#[derive(Debug)]
#[br(big)]
struct PageDefinition {
	start_id: u32,
	row_count: u32,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct LanguageDefinition {
	#[br(pad_after = 1)]
	language: u8,
	// unknown1: u8, //probably padding
}

impl LanguageDefinition {
	// TODO: Consider utilising some other data structure - realistically a bitfield
	// would be significantly smaller and more performant than a hash for this.
	fn to_set(languages: Vec<Self>) -> HashSet<u8> {
		languages.iter().map(|language| language.language).collect()
	}
}
