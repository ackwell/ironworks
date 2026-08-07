use std::io::{Read, Seek};

use crate::error::Result;

/// The `FormatRead` trait reads structured data from a provided source.
pub trait FormatRead: Sized {
	/// Construct an instance of `Self` from a reader.
	fn from_reader<R: Read + Seek>(reader: R) -> Result<Self>;
}

/// Extension methods for reading `FormatRead` values directly from a reader.
pub trait FileReaderExt: Read + Seek {
	/// Read a `T` value from the reader.
	#[inline]
	fn read_format<T: FormatRead>(&mut self) -> Result<T> {
		T::from_reader(self)
	}
}

impl<R> FileReaderExt for R where R: Read + Seek {}
