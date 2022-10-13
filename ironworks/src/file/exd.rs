//! Structs and utilities for parsing .exd files.

use std::{
	io::{Read, Seek, SeekFrom},
	sync::Mutex,
};

use binrw::{binread, BinRead};
use derivative::Derivative;

use crate::{
	error::{Error, ErrorValue, Result},
	FileStream,
};

use super::file::File;

#[binread]
#[derive(Debug)]
#[br(big, magic = b"EXDF")]
struct ExcelDataHeader {
	_version: u16,
	// unknown1: u16,
	#[br(pad_before = 2, temp)]
	index_size: u32,

	// unknown2: [u16; 10],
	#[br(
    pad_before = 20,
    count = index_size / RowDefinition::SIZE,
  )]
	rows: Vec<RowDefinition>,
}

/// An Excel data page. One or more pages form the full dataset for an Excel
/// sheet. Metadata for sheets is contained in an associated .exh Excel header file.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ExcelData {
	header: ExcelDataHeader,
	#[derivative(Debug = "ignore")]
	stream: Mutex<Box<dyn FileStream>>,
}

impl ExcelData {
	/// Fetch the slice of data associated with the specified row. If this data
	/// page is for a sheet with subrows, this will include all child rows of the
	/// specified row. Otherwise, it will contain the row and any trailing string data.
	pub fn row_data(&self, row_id: u32) -> Result<Vec<u8>> {
		let (row_header, offset) = self.row_meta(row_id)?;

		// Get a slice of the row's data
		let length: usize = row_header.data_size.try_into().unwrap();

		// TODO: this should be a helper
		let mut stream = self
			.stream
			.lock()
			.expect("TODO poisioned or contested read");
		let mut o = Vec::with_capacity(length);
		stream.seek(SeekFrom::Start(offset))?;
		stream
			.as_mut()
			.take(length.try_into().unwrap())
			.read_to_end(&mut o)?;

		// Ok(&self.data[offset..offset + length])
		Ok(o)
	}

	/// Fetch the number of subrows associated with a row ID. On sheets with a kind
	/// other than subrow, this will always be 1.
	pub fn subrow_count(&self, row_id: u32) -> Result<u16> {
		let (row_header, _offset) = self.row_meta(row_id)?;
		Ok(row_header.row_count)
	}

	/// Fetch the slice of data associated with the specified subrow.
	pub fn subrow_data(&self, row_id: u32, subrow_id: u16) -> Result<Vec<u8>> {
		let (row_header, offset) = self.row_meta(row_id)?;

		let error_value = || ErrorValue::Row {
			row: row_id,
			subrow: subrow_id,
			sheet: None,
		};

		// Double check the requested subrow is within the expected bounds.
		if subrow_id >= row_header.row_count {
			return Err(Error::NotFound(error_value()));
		}

		// Subrows invariably do not support unstructured data (i.e. strings), and
		// are laid out in subrow order. As such, it's safe to assume that evenly
		// splitting the row's data by it's subrow count will give us what we want.
		let subrow_size = u64::from(row_header.data_size / u32::from(row_header.row_count));
		let offset = offset + subrow_size * u64::from(subrow_id);

		// Sanity check the subrow header before returning.
		// let mut cursor = Cursor::new(&self.data);
		let mut stream = self
			.stream
			.lock()
			.expect("TODO poisioned or contested read");
		stream.seek(SeekFrom::Start(offset))?;
		let subrow_header = SubrowHeader::read(&mut stream.as_mut())?;

		if subrow_header.id != subrow_id {
			return Err(Error::Invalid(
				error_value(),
				format!("Subrow data reports as unexpected ID {}.", subrow_header.id),
			));
		}

		// Get the slice of subrow data.
		// TODO: this should be a helper
		let length = subrow_size - SubrowHeader::SIZE;
		let mut o = Vec::with_capacity(length.try_into().unwrap());
		stream.seek(SeekFrom::Start(offset + SubrowHeader::SIZE))?;
		stream.as_mut().take(length).read_to_end(&mut o)?;

		// Ok(&self.data[offset + SubrowHeader::SIZE..offset + subrow_size])
		Ok(o)
	}

	fn row_meta(&self, row_id: u32) -> Result<(RowHeader, u64)> {
		// Find the row definition for the requested row ID.
		let row_definition = self
			.header
			.rows
			.iter()
			.find(|row| row.id == row_id)
			.ok_or({
				Error::NotFound(ErrorValue::Row {
					row: row_id,
					subrow: 0,
					sheet: None,
				})
			})?;

		let mut stream = self
			.stream
			.lock()
			.expect("TODO poisioned or contested read");
		// Get a cursor to the start of the row.
		stream.seek(SeekFrom::Start(u64::from(row_definition.offset)))?;

		// Read in the header.
		let row_header = RowHeader::read(&mut stream.as_mut())?;
		let offset = stream.stream_position()?;

		Ok((row_header, offset))
	}
}

impl File for ExcelData {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		let header = ExcelDataHeader::read(&mut stream)?;
		Ok(Self {
			header,
			stream: Mutex::new(Box::new(stream)),
		})
	}
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct RowDefinition {
	id: u32,
	offset: u32,
}

impl RowDefinition {
	const SIZE: u32 = 8;
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct RowHeader {
	data_size: u32,
	row_count: u16,
}

#[binread]
#[derive(Debug)]
#[br(big)]
struct SubrowHeader {
	id: u16,
}

impl SubrowHeader {
	const SIZE: u64 = 2;
}
