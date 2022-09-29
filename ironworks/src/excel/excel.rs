use std::{convert::Infallible, fmt::Debug, sync::Arc};

use crate::{
	error::{Error, ErrorValue, Result},
	file,
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
	Ironworks,
};

use super::{
	borrowed::Borrowed,
	mapper::Mapper,
	metadata::SheetMetadata,
	sheet::{Sheet, SheetCache},
};

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
	pub fn build(
		&self,
		ironworks: impl Into<Borrowed<'i, Ironworks>>,
		mapper: impl Mapper + 'static,
	) -> Excel<'i> {
		Excel::with_options(ironworks, mapper, self)
	}
}

/// An Excel database.
pub struct Excel<'i> {
	default_language: u8,

	ironworks: Borrowed<'i, Ironworks>,
	mapper: Box<dyn Mapper>,

	list: OptionCache<file::exl::ExcelList>,
	sheets: HashMapCache<String, SheetCache>,
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
	pub fn new(
		ironworks: impl Into<Borrowed<'i, Ironworks>>,
		mapper: impl Mapper + 'static,
	) -> Self {
		Self::with().build(ironworks, mapper)
	}

	/// Build an Excel database with additional options.
	pub fn with() -> ExcelOptions {
		Default::default()
	}

	fn with_options(
		ironworks: impl Into<Borrowed<'i, Ironworks>>,
		mapper: impl Mapper + 'static,
		options: &ExcelOptions,
	) -> Self {
		Self {
			default_language: options.language.unwrap_or(0),

			ironworks: ironworks.into(),
			mapper: Box::new(mapper),

			list: Default::default(),
			sheets: Default::default(),
		}
	}

	/// Get the version string of the database.
	pub fn version(&self) -> Result<String> {
		self.ironworks.version(&self.mapper.exl())
	}

	/// Fetch the authoritative list of sheets in the database.
	pub fn list(&self) -> Result<Arc<file::exl::ExcelList>> {
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

		let cache = self
			.sheets
			.try_get_or_insert(sheet_name, || -> Result<_, Infallible> {
				Ok(Default::default())
			})
			.unwrap();

		Ok(Sheet::new(
			sheet_metadata,
			self.default_language,
			self.ironworks.clone(),
			self.mapper.as_ref(),
			cache,
		))
	}
}
