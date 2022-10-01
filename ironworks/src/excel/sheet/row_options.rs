use crate::{error::Result, excel::metadata::SheetMetadata};

use super::{sheet::Sheet, SheetIterator};

// TODO: rename? RowBuilder?
/// Options used when reading a row from a sheet.
#[derive(Debug)]
pub struct RowOptions<'s, S> {
	sheet: Option<&'s Sheet<'s, S>>,
	pub(super) config: RowConfig,
}

#[derive(Debug, Default, Clone)]
pub struct RowConfig {
	pub language: Option<u8>,
}

impl<'s, S: SheetMetadata> RowOptions<'s, S> {
	pub(super) fn new(sheet: &'s Sheet<S>) -> Self {
		Self {
			sheet: Some(sheet),
			config: Default::default(),
		}
	}

	/// Set the language to fetch.
	pub fn language(&mut self, language: impl Into<u8>) -> &mut Self {
		self.config.language = Some(language.into());
		self
	}

	/// Fetch a row from the sheet by ID. If the sheet supports subrows, this will
	/// return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<S::Row> {
		self.sheet().row_with_options(row_id, self.config.clone())
	}

	/// Fetch a subrow from the sheet by ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<S::Row> {
		self.sheet()
			.subrow_with_options(row_id, subrow_id, self.config.clone())
	}

	/// Iterate over the rows in this sheet.
	pub fn iter(&'s self) -> SheetIterator<'s, S> {
		self.sheet().iter_with_options(self.config.clone())
	}

	fn sheet(&self) -> &Sheet<'s, S> {
		self.sheet
			.expect("RowOptions created outside a sheet must be passed to a sheet manually.")
	}
}

impl<S> Default for RowOptions<'_, S> {
	fn default() -> Self {
		Self {
			sheet: None,
			config: Default::default(),
		}
	}
}
