use std::{convert::Infallible, error::Error};

use super::Row;

/// Metadata containing how to find and read an Excel sheet.
pub trait SheetMetadata {
	/// Name of the sheet.
	fn name(&self) -> String;

	/// Type of a successfully populated sheet row.
	type Row;
	/// Type of a failed population attempt.
	type Error: Error;
	/// Populate a sheet from the provided row reader.
	fn populate_row(&self, row: Row) -> Result<Self::Row, Self::Error>;
}

/// Implementation of sheet metadata for plain strings. This is used when the
/// sheet name is passed directly to the sheet method, and will result in direct
/// access to the row reader for manual field reading.
impl<S: ToString> SheetMetadata for S {
	fn name(&self) -> String {
		self.to_string()
	}

	type Row = Row;
	type Error = Infallible;
	fn populate_row(&self, row: Row) -> Result<Self::Row, Self::Error> {
		Ok(row)
	}
}
