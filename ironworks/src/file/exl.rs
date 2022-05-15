//! Structs and utilities for parsing .exl files.

use std::{borrow::Cow, collections::HashSet};

use crate::error::{Error, Result};

use super::File;

/// List of known Excel sheets.
#[derive(Debug)]
pub struct ExcelList {
	sheets: HashSet<String>,
}

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

impl File for ExcelList {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		let cow: Cow<[u8]> = data.into();

		// The excel list is actually just plaintext, read it in as a string.
		let list =
			String::from_utf8(cow.into_owned()).map_err(|error| Error::Resource(error.into()))?;

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
}

#[cfg(test)]
mod test {
	use crate::{error::Error, file::File};

	use super::ExcelList;

	const TEST_LIST: &[u8] = b"EXLT\r\nsheet1,0\r\nsheet2,0\r\nsheet3,0\r\n";

	#[test]
	fn empty() {
		let list = ExcelList::read(vec![]);
		assert!(matches!(list, Err(Error::Resource(_))));
	}

	#[test]
	fn missing_magic() {
		let list = ExcelList::read(b"hello\r\nworld".to_vec());
		assert!(matches!(list, Err(Error::Resource(_))));
	}

	#[test]
	fn has_sheet() {
		let list = ExcelList::read(TEST_LIST.to_vec()).unwrap();
		assert!(list.has("sheet2"));
	}

	#[test]
	fn missing_sheet() {
		let list = ExcelList::read(TEST_LIST.to_vec()).unwrap();
		assert!(!list.has("sheet4"));
	}
}
