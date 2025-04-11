//! Structs and utilities for parsing .exh files.

use binrw::{BinRead, NullString, binread};
use num_enum::IntoPrimitive;

use crate::{FileStream, error::Result};

use super::File;

/// An Excel header file, containing metadata for all associated .exd Excel data files.
#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXHF")]
pub struct ExcelHeader {
	/// File format version.
	pub version: u16,

	/// Size in bytes of fields for a single row in this sheet, or a single subrow
	/// for `SheetKind::Subrows`.
	pub row_size: u16,

	#[br(temp)]
	column_count: u16,
	#[br(temp)]
	page_count: u16,
	#[br(temp)]
	language_count: u16,

	///
	pub unknown1: u16,

	///
	pub unknown2: u8,

	/// The kind of the this sheet. This value dictates the binary layout and
	/// capabilities of rows.
	pub kind: SheetKind,

	///
	pub unknown3: u16,

	/// Total count of rows in this sheet across all pages, including subrows.
	pub row_count: u32,

	///
	pub unknown4: [u32; 2],

	/// Definition of the layout and type of columns.
	#[br(count = column_count)]
	pub columns: Vec<ColumnDefinition>,

	/// Definitions of the pages of data for this sheet
	#[br(count = page_count)]
	pub pages: Vec<PageDefinition>,

	/// IDs of languages supported by this sheet.
	#[br(
		count = language_count,
		map = |items: Vec<LanguageDefinition>| items.into_iter().map(|item| item.language).collect()
	)]
	pub languages: Vec<u8>,
}

impl File for ExcelHeader {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

/// The kind of sheet.
#[binread]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[br(repr = u8)]
pub enum SheetKind {
	/// Unknown kind. Will be treated equivalently to Default.
	Unknown = 0,

	/// Default sheet kind. Each row will contain one set of fields.
	Default = 1,

	/// Sheet with subrow support. Each row will contain one or more sets of
	/// fields, each with a discrete secondary ID. The (row_id, subrow_id) pair
	/// acts as a unique composite key.
	Subrows = 2,
}

/// Metadata for a single sheet column.
#[binread]
#[derive(Clone, Debug, Hash)]
#[br(big)]
pub struct ColumnDefinition {
	/// The kind of data stored in this column.
	pub kind: ColumnKind,

	/// The byte offset of this column within the row field data.
	pub offset: u16,
}

/// The kind of data structure stored in a column.
#[allow(missing_docs)]
#[binread]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive)]
#[br(big, repr = u16)]
#[repr(u16)]
pub enum ColumnKind {
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

/// Metadata for a single sheet data page.
#[binread]
#[derive(Debug, Clone, Copy)]
#[br(big)]
pub struct PageDefinition {
	/// The first ID contained within the page.
	pub start_id: u32,

	/// The number of rows contained within the page.
	pub row_count: u32,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct LanguageDefinition {
	language: u8,

	// Note: Seemingly unused?
	#[br(map = |raw: NullString| raw.to_string())]
	_unknown1: String,
}
