//! Structs and utilities for parsing .exl files.

use std::{borrow::Cow, collections::HashSet, io};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("bad magic: {0:?}")]
	BadMagic(String),

	#[error("I/O")]
	Io(#[source] io::Error),
}

/// List of known Excel sheets.
#[derive(Debug)]
pub struct ExcelList {
	sheets: HashSet<String>,
}

// TODO: should there be an impl intoiter for this?
impl ExcelList {
	/// Iterate over known sheets in arbitrary order.
	pub fn iter(&self) -> impl Iterator<Item = Cow<str>> {
		self.sheets.iter().map(|name| name.into())
	}

	/// Check if the specified sheet is contained in the list.
	pub fn has(&self, sheet: &str) -> bool {
		self.sheets.contains(sheet)
	}
}

impl ExcelList {
	pub fn read(mut stream: impl io::Read) -> Result<Self, Error> {
		// The excel list is actually just plaintext, read it in as a string.
		let mut list = String::new();
		stream.read_to_string(&mut list).map_err(Error::Io)?;

		let mut lines = list.split("\r\n");

		// Ensure the first line contains the expected magic
		let magic = lines.next().and_then(|line| line.get(0..4));
		if !matches!(magic, Some("EXLT")) {
			return Err(Error::BadMagic(magic.unwrap_or("").to_string()));
		}

		// Build the set of sheets. We're ignoring the sheet ID (second field), as
		// it's irrelevant for our usage.
		let sheets = lines
			.filter_map(|line| line.split_once(',').map(|split| split.0.to_string()))
			.collect::<HashSet<_>>();

		Ok(Self { sheets })
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
