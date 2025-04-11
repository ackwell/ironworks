//! Structs and utilities for parsing .exd files.

use std::io::Seek;

use binrw::{BinRead, BinResult, binread, error::CustomError};
use derivative::Derivative;

use crate::{FileStream, error::Result};

use super::file::File;

/// An Excel data page. One or more pages form the full dataset for an Excel
/// sheet. Metadata for sheets is contained in an associated .exh Excel header file.
#[binread]
#[derive(Derivative)]
#[derivative(Debug)]
#[br(big, magic = b"EXDF", stream = stream)]
pub struct ExcelData {
	/// EXD format version.
	pub version: u16,

	/// Unknown value. Only known to be 0, potentially padding.
	pub unknown1: u16,

	#[br(temp)]
	index_size: u32,

	#[br(temp)]
	data_size: u32,

	/// Unknown values. Only known to be 0.
	pub unknown2: [u16; 8],

	// Pre-emptively calculate where the data will start, so we can adjust row
	// offsets to be relative to it, rather than the start of the file.
	#[br(temp, calc = position::<u32>(stream)? + index_size)]
	data_offset: u32,

	/// Definitions for the rows contained in this page.
	#[br(
		count = usize::try_from(index_size).unwrap() / RowDefinition::SIZE,
		args { inner: (data_offset,) }
	)]
	pub rows: Vec<RowDefinition>,

	/// Buffer containing rows in this page.
	///
	/// Precise layout of rows varies based on information contained in this
	/// page's associated header file (`.exh`), including the sheet kind, and
	/// column layout.
	///
	/// **`exh::SheetKind::Default`**
	/// ```txt
	/// ┌───────────┬────────────┬───────────────┐
	/// │ RowHeader │ Field data │ String buffer │
	/// └───────────┴────────────┴───────────────┘
	/// ```
	///
	/// **`exh::SheetKind::Subrows`**
	/// ```txt
	/// ┌───────────┬──────────────┬────────────┬╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶┌───────────────┐
	/// │ RowHeader │ SubrowHeader │ Field data │ Further subrows │ String buffer │
	/// └───────────┴──────────────┴────────────┴╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶└───────────────┘
	/// ```
	#[br(count = data_size)]
	#[derivative(Debug = "ignore")]
	pub data: Vec<u8>,
}

impl File for ExcelData {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

/// Definition of a row within a page.
#[binread]
#[derive(Debug)]
#[br(big, import(data_offset: u32))]
pub struct RowDefinition {
	/// Primary key ID of this row.
	pub id: u32,

	/// Offset of this row within the data buffer.
	#[br(map = |raw: u32| raw - data_offset)]
	pub offset: u32,
}

impl RowDefinition {
	const SIZE: usize = 8;
}

/// Inlined metadata for a row within a page.
#[binread]
#[derive(Debug)]
#[br(big)]
pub struct RowHeader {
	/// Byte size of this row's data, including both fields and string buffer.
	pub size: u32,

	/// Number of subrows present within this row. Only meaningful for
	/// `SheetKind::Subrows`, will be `1` for other kinds.
	pub count: u16,
}

/// Inlined metadata for a subrow within a row.
#[binread]
#[derive(Debug)]
#[br(big)]
pub struct SubrowHeader {
	/// ID of this subrow within the parent row.
	pub id: u16,
}

impl SubrowHeader {
	/// Size of subrow header, in bytes.
	pub const SIZE: usize = 2;
}

/// Get the current byte offset of the stream.
///
/// Will fail if the offset cannot be converted to `T`.
fn position<T>(stream: &mut impl Seek) -> BinResult<T>
where
	u64: TryInto<T>,
	<u64 as TryInto<T>>::Error: CustomError + 'static,
{
	let position = stream.stream_position()?;
	position.try_into().map_err(|error| binrw::Error::Custom {
		pos: position,
		err: Box::new(error),
	})
}
