//! Structs and utilities for parsing .exl files.

use std::{borrow::Cow, collections::HashMap, num::ParseIntError};

use crate::{
	FileStream,
	error::{Error, Result},
};

use super::File;

/// List of known Excel sheets.
#[derive(Debug)]
pub struct ExcelList(pub HashMap<String, i32>);

// TODO: should there be an impl intoiter for this?
impl ExcelList {
	/// Iterate over known sheets in arbitrary order.
	pub fn iter(&self) -> impl Iterator<Item = (Cow<str>, i32)> {
		self.0.iter().map(|(name, id)| (name.into(), *id))
	}

	/// Check if the specified sheet is contained in the list.
	pub fn has(&self, sheet: &str) -> bool {
		self.0.contains_key(sheet)
	}
}

impl File for ExcelList {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		// The excel list is actually just plaintext, read it in as a string.
		let mut list = String::new();
		stream
			.read_to_string(&mut list)
			.map_err(|error| Error::Resource(error.into()))?;

		let mut lines = list.split("\r\n");

		// Ensure the first line contains the expected magic
		let magic = lines.next().and_then(|line| line.get(0..4));
		if !matches!(magic, Some("EXLT")) {
			return Err(Error::Resource(
				format!("Incorrect magic in excel list file: expected \"EXLT\", got {magic:?}")
					.into(),
			));
		}

		// Build the set of sheets.
		let sheets: Result<HashMap<String, i32>, ParseIntError> = lines
			.filter_map(|line| {
				line.split_once(',')
					.map(|(name, id)| Ok((name.to_string(), id.parse()?)))
			})
			.collect();

		match sheets {
			Err(error) => Err(Error::Resource(
				format!("Failed to parse excel list: {error}").into(),
			)),
			Ok(sheets) => Ok(Self(sheets)),
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::{self, Cursor};

	use crate::{error::Error, file::File};

	use super::ExcelList;

	const TEST_LIST: &[u8] = b"EXLT\r\nsheet1,0\r\nsheet2,0\r\nsheet3,0\r\n";

	#[test]
	fn empty() {
		let list = ExcelList::read(io::empty());
		assert!(matches!(list, Err(Error::Resource(_))));
	}

	#[test]
	fn missing_magic() {
		let list = ExcelList::read(Cursor::new(b"hello\r\nworld".to_vec()));
		assert!(matches!(list, Err(Error::Resource(_))));
	}

	#[test]
	fn has_sheet() {
		let list = ExcelList::read(Cursor::new(TEST_LIST)).unwrap();
		assert!(list.has("sheet2"));
	}

	#[test]
	fn missing_sheet() {
		let list = ExcelList::read(Cursor::new(TEST_LIST)).unwrap();
		assert!(!list.has("sheet4"));
	}
}
