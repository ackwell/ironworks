//! Structs and utilities for parsing .exh files.

use std::collections::HashSet;

use binrw::{binread, BinRead};
use getset::{CopyGetters, Getters};
use num_enum::IntoPrimitive;

use crate::{error::Result, FileStream};

use super::File;

/// An Excel header file, containing metadata for all associated .exd Excel data files.
#[binread]
#[derive(Debug, Getters, CopyGetters)]
#[br(big, magic = b"EXHF")]
pub struct ExcelHeader {
	_version: u16,

	/// Size of structured data in each row, in bytes.
	#[get_copy = "pub"]
	row_size: u16,

	#[br(temp)]
	column_count: u16,
	#[br(temp)]
	page_count: u16,
	#[br(temp)]
	language_count: u16,

	// unknown1: u16,
	// unknown2: u8,
	/// The kind of the relevant sheet. This value dictates the binary layout and
	/// capabilities of rows.
	#[br(pad_before = 3)]
	#[get_copy = "pub"]
	kind: SheetKind,

	// unknown3: u16,
	#[br(pad_before = 2)]
	_row_count: u32,

	// unknown4: [u32; 2],
	/// Column definitions for rows in this sheet.
	#[br(
		pad_before = 8,
		count = column_count,
	)]
	#[get = "pub"]
	columns: Vec<ColumnDefinition>,

	/// Definitions of the pages of data for this sheet.
	#[br(count = page_count)]
	#[get = "pub"]
	pages: Vec<PageDefinition>,

	/// Language IDs supported by this sheet.
	#[br(
		count = language_count,
		map = LanguageDefinition::to_set,
	)]
	#[get = "pub"]
	languages: HashSet<u8>,
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

	/// Default sheet kind. Supports string payloads. Strings are stored in data
	/// immediately following the end of the structured row segment.
	Default = 1,

	/// Subrow sheet. Each row may have one or more subrows, IDs acting as a
	/// secondary key. Subrow sheets do not support string payloads.
	Subrows = 2,
}

/// Metadata for a single sheet column.
#[binread]
#[derive(Clone, Debug, Hash, CopyGetters)]
#[br(big)]
pub struct ColumnDefinition {
	/// The kind of data stored in this column.
	#[get_copy = "pub"]
	kind: ColumnKind,

	/// The offset of this column in bytes within the row structured data.
	#[get_copy = "pub"]
	offset: u16,
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
#[derive(Debug, Clone, Copy, CopyGetters)]
#[br(big)]
pub struct PageDefinition {
	/// The first ID contained within the page.
	#[get_copy = "pub"]
	start_id: u32,

	/// The number of rows contained within the page.
	#[get_copy = "pub"]
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
