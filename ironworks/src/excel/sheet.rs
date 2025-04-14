use std::{
	collections::HashMap,
	sync::{Arc, OnceLock, RwLock},
};

use derivative::Derivative;
use num_enum::FromPrimitive;

use crate::{
	error::{Error, ErrorValue, Result},
	file::{exd, exh},
	ironworks::Ironworks,
};

use super::{
	language::Language,
	metadata::SheetMetadata,
	page::{Page, PageIterator, RowSpecifier, SubrowSpecifier},
	path,
	row::Row,
};

/// A sheet within an Excel database.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Sheet<S> {
	#[derivative(Debug = "ignore")]
	ironworks: Arc<Ironworks>,

	metadata: S,
	pub(super) default_language: Language,

	#[derivative(Debug = "ignore")]
	cache: Arc<SheetCache>,
}

impl<S: SheetMetadata> Sheet<S> {
	pub(crate) fn new(
		ironworks: Arc<Ironworks>,
		metadata: S,
		default_language: Language,
		cache: Arc<SheetCache>,
	) -> Self {
		Self {
			ironworks,
			metadata,
			default_language,
			cache,
		}
	}

	/// Set the default language to use when reading from this sheet.
	pub fn with_default_language(mut self, default_language: Language) -> Self {
		self.set_default_language(default_language);
		self
	}

	/// Set the default language to use when reading from this sheet.
	pub fn set_default_language(&mut self, default_language: Language) {
		self.default_language = default_language;
	}

	/// Name of the sheet as specified by the provided metadata.
	pub fn name(&self) -> String {
		self.metadata.name()
	}

	/// Get the kind of this sheet.
	pub fn kind(&self) -> Result<exh::SheetKind> {
		Ok(self.header()?.kind)
	}

	/// List of languages supported by this sheet.
	pub fn languages(&self) -> Result<Vec<Language>> {
		let languages = self
			.header()?
			.languages
			.iter()
			.copied()
			.map(Language::from_primitive)
			.collect();

		Ok(languages)
	}

	/// Fetch metadata for all columns in this sheet.
	pub fn columns(&self) -> Result<Vec<exh::ColumnDefinition>> {
		let columns = self.header()?.columns.clone();
		Ok(columns)
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<S::Row> {
		self.row_with_options(row_id, RowOptions::new())
	}

	/// Fetch a row from this sheet by ID, along with any additional options for
	/// reading the row. In the case of a sheet with subrows, this will return subrow 0.
	pub fn row_with_options(&self, row_id: u32, options: impl Into<RowOptions>) -> Result<S::Row> {
		self.subrow_with_options(row_id, 0, options)
	}

	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<S::Row> {
		self.subrow_with_options(row_id, subrow_id, RowOptions::new())
	}

	/// Fetch a row from this sheet by its ID and subrow ID, along with any additional options for reading the row.
	pub fn subrow_with_options(
		&self,
		row_id: u32,
		subrow_id: u16,
		options: impl Into<RowOptions>,
	) -> Result<S::Row> {
		let options: RowOptions = options.into();
		let sheet_header = self.header()?;

		let row_error_value = || ErrorValue::Row {
			row: row_id,
			subrow: subrow_id,
			sheet: self.name().into(),
		};

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if sheet_header.kind != exh::SheetKind::Subrows && subrow_id > 0 {
			return Err(Error::NotFound(row_error_value()));
		}

		// Try to read in the page for the requested (sub)row.
		let start_id = self
			.start_id_for_row(row_id)
			.ok_or_else(|| Error::NotFound(row_error_value()))?;
		let language = self.resolve_language(options.language.unwrap_or(self.default_language))?;
		let page = self.page(start_id, language)?;

		let row = page.row(RowSpecifier::Id(row_id), SubrowSpecifier::Id(subrow_id))?;

		self.populate_row(row)
	}

	fn populate_row(&self, row: Row) -> Result<S::Row> {
		let row_id = row.row_id();
		let subrow_id = row.subrow_id();

		self.metadata.populate_row(row).map_err(|error| {
			Error::Invalid(
				ErrorValue::Row {
					row: row_id,
					subrow: subrow_id,
					sheet: Some(self.name()),
				},
				error.to_string(),
			)
		})
	}

	pub(super) fn header(&self) -> Result<Arc<exh::ExcelHeader>> {
		// TODO: get_or_try_init once (if?) that gets stabilised.
		if let Some(header) = self.cache.header.get() {
			return Ok(header.clone());
		}

		let path = path::exh(&self.name());
		let header = self.ironworks.file(&path)?;

		Ok(self.cache.header.get_or_init(|| Arc::new(header)).clone())
	}

	fn start_id_for_row(&self, row_id: u32) -> Option<u32> {
		let header = self.header().ok()?;

		header
			.pages
			.iter()
			.find(|page| page.start_id <= row_id && page.start_id + page.row_count > row_id)
			.map(|page| page.start_id)
	}

	pub(super) fn page(&self, start_id: u32, language: Language) -> Result<Arc<Page>> {
		let key = (start_id, language);

		// Try to fetch from the hot path.
		let pages = self.cache.pages.read().expect("poisoned");
		if let Some(page) = pages.get(&key) {
			return Ok(page.clone());
		}

		// No page already present, take ownership over reading + caching it.
		// This is likely slightly susceptible to a race, but that's a lot cheaper than a mutex.
		drop(pages);
		let mut pages_mut = self.cache.pages.write().expect("poisoned");

		let path = path::exd(&self.name(), start_id, language)?;
		let page = Arc::new(Page::new(
			self.header()?,
			self.ironworks.file::<exd::ExcelData>(&path)?,
		));

		pages_mut.insert(key, page.clone());

		Ok(page)
	}

	pub(super) fn resolve_language(&self, language: Language) -> Result<Language> {
		let header = self.header()?;

		// Get the language to load, or None if the language is not supported by this sheet.
		// TODO: Should an explicit language request fail hard on miss?
		[language, Language::None]
			.into_iter()
			.find(|&language| header.languages.contains(&language.into()))
			// TODO: Should this be Invalid or NotFound?
			// TODO: Should we have an explicit ErrorValue for language?
			.ok_or_else(|| Error::NotFound(ErrorValue::Other(format!("language {language:?}"))))
	}
}

impl<S: SheetMetadata> IntoIterator for Sheet<S> {
	type Item = Result<S::Row>;
	type IntoIter = SheetIterator<S>;

	fn into_iter(self) -> Self::IntoIter {
		SheetIterator::new(self)
	}
}

/// Data cache for raw values, decoupled from mapping/metadata concerns.
#[derive(Default)]
pub struct SheetCache {
	header: OnceLock<Arc<exh::ExcelHeader>>,
	pages: RwLock<HashMap<(u32, Language), Arc<Page>>>,
}

/// Options used when reading a row from a sheet.
#[derive(Debug)]
pub struct RowOptions {
	language: Option<Language>,
}

impl RowOptions {
	/// Build the default options for reading a row.
	pub fn new() -> Self {
		Self { language: None }
	}
}

impl From<Language> for RowOptions {
	fn from(language: Language) -> Self {
		Self {
			language: Some(language),
		}
	}
}

/// Iterator over the rows in a sheet.
#[derive(Debug)]
pub struct SheetIterator<S> {
	sheet: Sheet<S>,

	stop_iteration: bool,

	page_iter: Option<PageIterator>,
	next_page_index: usize,
}

impl<S> SheetIterator<S> {
	fn new(sheet: Sheet<S>) -> Self {
		Self {
			sheet,
			stop_iteration: false,
			page_iter: None,
			next_page_index: 0,
		}
	}
}

impl<S: SheetMetadata> Iterator for SheetIterator<S> {
	type Item = Result<S::Row>;

	fn next(&mut self) -> Option<Self::Item> {
		// If this iterator has failed in a way that iteration can't meaningfully
		// continue, close out immediately.
		if self.stop_iteration {
			return None;
		}

		loop {
			// Try to get the next item from the current page iterator, if any exists.
			if let Some(next) = self.page_iter.as_mut().and_then(|iter| iter.next()) {
				let mapped = next.and_then(|row| self.sheet.populate_row(row));
				return Some(mapped);
			}

			// Nothing was obtained - try to move to the next page iterator.
			// If the header cannot be retrieved, there's no way for us to continue iterating at all.
			let header = match self.sheet.header() {
				Ok(header) => header,
				Err(error) => {
					self.stop_iteration = true;
					return Some(Err(error));
				}
			};

			// Get the next page definition - if this is out of bounds, we're at end
			// of iteration and can shortcut the None out.
			let definition = header.pages.get(self.next_page_index)?;
			self.next_page_index += 1;

			// Similar to the header, we need the language.
			let language = match self.sheet.resolve_language(self.sheet.default_language) {
				Ok(language) => language,
				Err(error) => {
					self.stop_iteration = true;
					return Some(Err(error));
				}
			};

			// If the page can't be fetched, we may be able to move on to the next
			// page - it could be an issue with a single page file.
			let page = match self.sheet.page(definition.start_id, language) {
				Ok(page) => page,
				Err(error) => return Some(Err(error)),
			};

			self.page_iter = Some(PageIterator::new(page));
		}
	}
}
