use std::{
	convert::Infallible,
	sync::{Arc, OnceLock},
};

use derivative::Derivative;

use crate::{
	error::{Error, ErrorValue, Result},
	file::exl,
	ironworks::Ironworks,
	utility::{HashMapCache, HashMapCacheExt},
};

use super::{
	language::Language,
	metadata::SheetMetadata,
	path,
	sheet::{Sheet, SheetCache},
};

/// An Excel database.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Excel {
	#[derivative(Debug = "ignore")]
	ironworks: Arc<Ironworks>,

	default_language: Language,

	#[derivative(Debug = "ignore")]
	list: OnceLock<exl::ExcelList>,
	#[derivative(Debug = "ignore")]
	sheets: HashMapCache<String, SheetCache>,
}

impl Excel {
	/// Build an view into the Excel database for a given ironworks instance.
	pub fn new(ironworks: impl Into<Arc<Ironworks>>) -> Self {
		Self {
			ironworks: ironworks.into(),

			default_language: Language::None,

			list: Default::default(),
			sheets: Default::default(),
		}
	}

	/// Set the default language to use when reading from the database.
	pub fn with_default_language(mut self, language: Language) -> Self {
		self.set_default_language(language);
		self
	}

	/// Set the default language to use when reading from the database.
	pub fn set_default_language(&mut self, language: Language) {
		self.default_language = language;
	}

	/// Get the version string of the database.
	pub fn version(&self) -> Result<String> {
		self.ironworks.version(path::exl())
	}

	/// Fetch the authoritative list of sheets in the database.
	pub fn list(&self) -> Result<&exl::ExcelList> {
		// Handle hot path before trying anything fancy.
		// We're doing this rather than executing .file inside .get_or_init to avoid caching error states.
		// TODO: get_or_try_init once (if?) that gets stabilised.
		if let Some(list) = self.list.get() {
			return Ok(list);
		}

		let list = self.ironworks.file::<exl::ExcelList>(path::exl())?;

		Ok(self.list.get_or_init(|| list))
	}

	/// Fetch a sheet from the database.
	pub fn sheet<S: SheetMetadata>(&self, metadata: S) -> Result<Sheet<S>> {
		let name = metadata.name();

		let list = self.list()?;
		if !list.has(&name) {
			return Err(Error::NotFound(ErrorValue::Sheet(name)));
		}

		let cache = self
			.sheets
			.try_get_or_insert(name, || -> Result<_, Infallible> { Ok(Default::default()) })
			.unwrap();

		Ok(Sheet::new(
			self.ironworks.clone(),
			metadata,
			self.default_language,
			cache,
		))
	}
}
