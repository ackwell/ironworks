//! Structs and utilities for parsing .exd files.

use binrw::{BinRead, BinResult, binread, error::CustomError, parser};
use derivative::Derivative;

use crate::{FileStream, error::Result};

use super::file::File;

/// An Excel data page. One or more pages form the full dataset for an Excel
/// sheet. Metadata for sheets is contained in an associated .exh Excel header file.
#[binread]
#[derive(Derivative)]
#[derivative(Debug)]
#[br(big, magic = b"EXDF")]
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
	#[br(temp, parse_with = current_position)]
	index_offset: u32,

	#[br(temp, calc = index_offset + index_size)]
	data_offset: u32,

	/// Definitions for the rows contained in this page.
	#[br(
		count = usize::try_from(index_size).unwrap() / RowDefinition::SIZE,
		args { inner: (data_offset,) }
	)]
	pub rows: Vec<RowDefinition>,

	#[br(count = data_size)]
	#[derivative(Debug = "ignore")]
	pub data: Vec<u8>,
}

impl File for ExcelData {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

/// Metadata of a row contained in a page.
#[binread]
#[derive(Debug)]
#[br(big, import(data_offset: u32))]
pub struct RowDefinition {
	/// Primary key ID of this row.
	pub id: u32,

	#[br(map = |raw: u32| raw - data_offset)]
	pub offset: u32,
}

impl RowDefinition {
	const SIZE: usize = 8;
}

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct RowHeader {
	pub size: u32,
	// todo doc: subrow count - present even on non-subrow-sheets
	pub count: u16,
}

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct SubrowHeader {
	pub id: u16,
}

impl SubrowHeader {
	pub const SIZE: usize = 2;
}

/// Get the current byte offset of the stream.
///
/// Will fail if the offset cannot be converted to `T`.
#[parser(reader)]
fn current_position<T>() -> BinResult<T>
where
	u64: TryInto<T>,
	<u64 as TryInto<T>>::Error: CustomError + 'static,
{
	let position = reader.stream_position()?;
	position.try_into().map_err(|error| binrw::Error::Custom {
		pos: position,
		err: Box::new(error),
	})
}
