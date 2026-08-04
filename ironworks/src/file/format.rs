use std::io::{Read, Seek};

use crate::error::Result;

/// The `FormatRead` trait reads structured data from a provided source.
pub trait FormatRead: Sized {
	/// Construct an instance of `Self` from a reader.
	fn from_reader<R: Read + Seek>(reader: R) -> Result<Self>;
}

impl FormatRead for Vec<u8> {
	fn from_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
		let mut buffer = Vec::new();
		reader.read_to_end(&mut buffer)?;
		Ok(buffer)
	}
}
