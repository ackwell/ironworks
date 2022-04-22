use crate::{
	error::Result,
	excel::{metadata::SheetMetadata, Resource},
};

use super::sheet::Sheet;

/// Options used when reading a row from a sheet.
#[derive(Debug)]
pub struct RowOptions<'s, S, R> {
	sheet: Option<&'s Sheet<'s, S, R>>,
	pub(super) language: Option<u8>,
}

impl<'s, S: SheetMetadata, R: Resource> RowOptions<'s, S, R> {
	pub(super) fn new(sheet: &'s Sheet<S, R>) -> Self {
		Self {
			sheet: Some(sheet),
			language: None,
		}
	}

	/// Set the language to fetch.
	pub fn language(&mut self, language: impl Into<u8>) -> &mut Self {
		self.language = Some(language.into());
		self
	}

	/// Fetch a row from the sheet by ID. If the sheet supports subrows, this will
	/// return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<S::Row> {
		self.sheet().row_with_options(row_id, self)
	}

	/// Fetch a subrow from the sheet by ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<S::Row> {
		self.sheet().subrow_with_options(row_id, subrow_id, self)
	}

	fn sheet(&self) -> &Sheet<'s, S, R> {
		self.sheet
			.expect("RowOptions created outside a sheet must be passed to a sheet manually.")
	}
}

impl<S, R> Default for RowOptions<'_, S, R> {
	fn default() -> Self {
		Self {
			sheet: None,
			language: None,
		}
	}
}
