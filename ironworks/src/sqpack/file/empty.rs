use std::io::{Empty, Read, Seek};

use crate::error::{Error, ErrorValue, Result};

use super::shared::Header;

pub fn read(reader: impl Read + Seek, header: Header) -> Result<Empty> {
	let mut buf = Vec::with_capacity(header.raw_file_size.try_into().unwrap());
	reader
		.take(header.raw_file_size.into())
		.read_to_end(&mut buf)?;

	// TODO: if type 1 and first 64 == second 64, RSF
	//       if type 1 and first 64 == [0..], empty

	// Empty files can't be read as-is - they're either entirely invalid, or need
	// further processing that doesn't belong in sqpack specifically.
	Err(Error::Invalid(
		ErrorValue::File(buf),
		String::from("Empty file"),
	))
}
