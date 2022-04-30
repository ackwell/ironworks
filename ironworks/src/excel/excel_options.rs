use crate::Ironworks;

use super::excel::Excel;

/// Options for the root Excel database.
#[derive(Debug, Default)]
pub struct ExcelOptions {
	pub(super) language: Option<u8>,
}

impl<'i> ExcelOptions {
	/// Set the default language of the database
	pub fn language(&mut self, language: impl Into<u8>) -> &mut Self {
		self.language = Some(language.into());
		self
	}

	/// Build the configured Excel database.
	pub fn build(&self, ironworks: &'i Ironworks) -> Excel<'i> {
		Excel::with_options(ironworks, self)
	}
}
