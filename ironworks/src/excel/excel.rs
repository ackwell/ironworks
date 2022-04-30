use std::{fmt::Debug, sync::Arc};

use crate::{
	error::{Error, ErrorValue, Result},
	file,
	utility::{OptionCache, OptionCacheExt},
	Ironworks,
};

use super::{excel_options::ExcelOptions, mapper::Mapper, metadata::SheetMetadata, sheet::Sheet};

/// An Excel database.
pub struct Excel<'i> {
	default_language: u8,

	ironworks: &'i Ironworks,
	mapper: Box<dyn Mapper>,

	list: OptionCache<file::exl::Exl>,
}

impl Debug for Excel<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Excel")
			.field("default_language", &self.default_language)
			.finish()
	}
}

impl<'i> Excel<'i> {
	/// Build an Excel database.
	pub fn new(ironworks: &'i Ironworks, mapper: impl Mapper + 'static) -> Self {
		Self::with().build(ironworks, mapper)
	}

	/// Build an Excel database with additional options.
	pub fn with() -> ExcelOptions {
		Default::default()
	}

	pub(super) fn with_options(
		ironworks: &'i Ironworks,
		mapper: impl Mapper + 'static,
		options: &ExcelOptions,
	) -> Self {
		Self {
			default_language: options.language.unwrap_or(0),

			ironworks,
			mapper: Box::new(mapper),

			list: Default::default(),
		}
	}

	/// Get the version string of the database.
	pub fn version(&self) -> Result<String> {
		// self.resource.version()
		Ok("TODO".into())
	}

	/// Fetch the authoritative list of sheets in the database.
	pub fn list(&self) -> Result<Arc<file::exl::Exl>> {
		// TODO: name mapping to decouple xiv
		self.list
			.try_get_or_insert(|| self.ironworks.file(&self.mapper.exl()))
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
			self.mapper.as_ref(),
		))
	}
}
