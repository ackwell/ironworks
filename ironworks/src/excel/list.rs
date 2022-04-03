use std::{collections::HashSet, io::Read};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct List {
	sheets: HashSet<String>,
}

impl List {
	pub fn read<R: Read>(mut reader: R) -> Result<Self> {
		// The excel list is actually just plaintext, read it in as a string.
		let mut list = String::new();
		reader
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

		// Build the set of sheets. We're ignoring the sheet ID (second field), as
		// it's irrelevant for our usage.
		let sheets = lines
			.filter_map(|line| line.split_once(',').map(|split| split.0.to_string()))
			.collect::<HashSet<_>>();

		Ok(Self { sheets })
	}

	pub fn has(&self, sheet: &str) -> bool {
		self.sheets.contains(sheet)
	}
}

#[cfg(test)]
mod test {
	use std::io;

	use crate::error::Error;

	use super::List;

	const TEST_LIST: &[u8] = b"EXLT\r\nsheet1,0\r\nsheet2,0\r\nsheet3,0\r\n";

	#[test]
	fn empty() {
		let list = List::read(io::empty());
		assert!(matches!(list, Err(Error::Resource(_))));
	}

	#[test]
	fn missing_magic() {
		let list = List::read(io::Cursor::new(b"hello\r\nworld"));
		assert!(matches!(list, Err(Error::Resource(_))));
	}

	#[test]
	fn has_sheet() {
		let list = List::read(io::Cursor::new(TEST_LIST)).unwrap();
		assert!(list.has("sheet2"));
	}

	#[test]
	fn missing_sheet() {
		let list = List::read(io::Cursor::new(TEST_LIST)).unwrap();
		assert!(!list.has("sheet4"));
	}
}
