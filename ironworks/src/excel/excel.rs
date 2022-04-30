use std::sync::Arc;

use crate::{
	error::{Error, ErrorValue, Result},
	utility::{OptionCache, OptionCacheExt},
	Ironworks,
};

use super::{excel_options::ExcelOptions, list::List, metadata::SheetMetadata, sheet::Sheet};

/// An Excel database.
#[derive(Debug)]
pub struct Excel<'i> {
	default_language: u8,

	ironworks: &'i Ironworks,

	list: OptionCache<List>,
}

impl<'i> Excel<'i> {
	/// Build an Excel database.
	pub fn new(ironworks: &'i Ironworks) -> Self {
		Self::with().build(ironworks)
	}

	/// Build an Excel database with additional options.
	pub fn with() -> ExcelOptions {
		Default::default()
	}

	pub(super) fn with_options(ironworks: &'i Ironworks, options: &ExcelOptions) -> Self {
		Self {
			default_language: options.language.unwrap_or(0),

			ironworks,

			list: Default::default(),
		}
	}

	/// Get the version string of the database.
	pub fn version(&self) -> Result<String> {
		// self.resource.version()
		Ok("TODO".into())
	}

	/// Fetch the authoratative list of sheets in the database.
	pub fn list(&self) -> Result<Arc<List>> {
		// TODO: name mapping to decouple xiv
		self.list
			.try_get_or_insert(|| self.ironworks.file("exd/root.exl"))
	}

	/// Fetch a sheet from the database.
	pub fn sheet<S: SheetMetadata>(&self, sheet_metadata: S) -> Result<Sheet<S>> {
		let sheet_name = sheet_metadata.name();

		let list = self.list()?;
		if !list.has(&sheet_name) {
			return Err(Error::NotFound(ErrorValue::Sheet(sheet_name)));
		}

		Ok(Sheet::new(
			sheet_metadata,
			self.default_language,
			self.ironworks,
		))
	}
}
