use std::{collections::HashSet, io::Read};

use crate::error::{Error, Result};

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
		let magic = lines.next().map(|line| &line[0..4]);
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
