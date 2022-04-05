use crate::{error::Result, excel::Resource};

use super::sheet::Sheet;

/// Options used when reading a row from a sheet.
#[derive(Debug)]
pub struct RowOptions<'s, R> {
	sheet: Option<&'s Sheet<'s, R>>,
	pub(super) language: Option<u8>,
}

impl<'s, R: Resource> RowOptions<'s, R> {
	pub(super) fn new(sheet: &'s Sheet<R>) -> Self {
		Self {
			sheet: Some(sheet),
			language: None,
		}
	}

	/// Set the language to fetch.
	pub fn language(&mut self, language: u8) -> &mut Self {
		self.language = Some(language);
		self
	}

	/// Fetch a row from the sheet by ID. If the sheet supports subrows, this will
	/// return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<()> {
		self.sheet().row_with_options(row_id, self)
	}

	/// Fetch a subrow from the sheet by ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<()> {
		self.sheet().subrow_with_options(row_id, subrow_id, self)
	}

	fn sheet(&self) -> &Sheet<'s, R> {
		self.sheet
			.expect("RowOptions created outside a sheet must be passed to a sheet manually.")
	}
}

impl<R> Default for RowOptions<'_, R> {
	fn default() -> Self {
		Self {
			sheet: None,
			language: None,
		}
	}
}
