use std::{fmt::Debug, sync::Arc};

use num_enum::TryFromPrimitive;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::{borrowed::Borrowed, language::Language, metadata::SheetMetadata, path, row::Row},
	file::{exd, exh},
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
	Ironworks,
};

use super::{
	row_options::{RowConfig, RowOptions},
	SheetIterator,
};

// TODO: how much should be in this? Arguably the mapper &co might be relevant given that the mapper is required to fill the caches, etc.
/// Data cache for raw values, decoupled from mapping/metadata concerns.
#[derive(Debug, Default)]
pub struct SheetCache {
	header: OptionCache<exh::ExcelHeader>,
	pages: HashMapCache<(u32, Language), exd::ExcelData>,
}

/// A sheet within an Excel database.
pub struct Sheet<'i, S> {
	sheet_metadata: S,
	default_language: Language,

	ironworks: Borrowed<'i, Ironworks>,

	cache: Arc<SheetCache>,
}

impl<S: Debug> Debug for Sheet<'_, S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Sheet")
			.field("sheet_metadata", &self.sheet_metadata)
			.field("default_language", &self.default_language)
			.finish()
	}
}

impl<'i, S: SheetMetadata> Sheet<'i, S> {
	pub(crate) fn new(
		sheet_metadata: S,
		default_language: Language,
		ironworks: Borrowed<'i, Ironworks>,
		cache: Arc<SheetCache>,
	) -> Self {
		Self {
			sheet_metadata,
			default_language,

			ironworks,

			cache,
		}
	}

	/// Name of the sheet as speified by the provided metadata.
	pub fn name(&self) -> String {
		self.sheet_metadata.name()
	}

	/// Get the kind of this sheet.
	pub fn kind(&self) -> Result<exh::SheetKind> {
		let kind = self.header()?.kind();
		Ok(kind)
	}

	/// List of languages supported by this sheet.
	pub fn languages(&self) -> Result<Vec<Language>> {
		self.header()?
			.languages()
			.iter()
			.copied()
			.map(Language::try_from_primitive)
			.collect::<Result<Vec<_>, _>>()
			.map_err(|err| {
				Error::Invalid(
					ErrorValue::Sheet(self.name()),
					format!("unknown language ID {}", err.number),
				)
			})
	}

	/// Fetch metadata for all columns in this sheet.
	pub fn columns(&self) -> Result<Vec<exh::ColumnDefinition>> {
		let columns = self.header()?.columns().clone();
		Ok(columns)
	}

	/// Create a row options builder for this sheet.
	pub fn with(&'i self) -> RowOptions<'i, S> {
		RowOptions::new(self)
	}

	/// Iterate over the rows in this sheet.
	pub fn iter(&'i self) -> SheetIterator<'i, S> {
		self.iter_with_options(Default::default())
	}

	pub(super) fn iter_with_options(&'i self, config: RowConfig) -> SheetIterator<'i, S> {
		SheetIterator::new(self, config)
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<S::Row> {
		self.row_with_options(row_id, Default::default())
	}

	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<S::Row> {
		self.subrow_with_options(row_id, subrow_id, Default::default())
	}

	pub(super) fn row_with_options(&self, row_id: u32, config: RowConfig) -> Result<S::Row> {
		self.subrow_with_options(row_id, 0, config)
	}

	pub(super) fn subrow_with_options(
		&self,
		row_id: u32,
		subrow_id: u16,
		config: RowConfig,
	) -> Result<S::Row> {
		let header = self.header()?;

		let row_error_value = || ErrorValue::Row {
			row: row_id,
			subrow: subrow_id,
			sheet: self.name().into(),
		};

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if header.kind() != exh::SheetKind::Subrows && subrow_id > 0 {
			return Err(Error::NotFound(row_error_value()));
		}

		// Try to read in the page for the requested (sub)row.
		let page = self.page_for_row(row_id, subrow_id, config.language)?;

		let data = match header.kind() {
			exh::SheetKind::Subrows => page.subrow_data(row_id, subrow_id),
			_ => page.row_data(row_id),
		}?;
		let row = Row::new(row_id, subrow_id, header, data.to_vec());

		self.sheet_metadata
			.populate_row(row)
			.map_err(|error| Error::Invalid(row_error_value(), error.to_string()))
	}

	pub(super) fn header(&self) -> Result<Arc<exh::ExcelHeader>> {
		self.cache.header.try_get_or_insert(|| {
			let path = path::exh(&self.name());
			self.ironworks.file(&path)
		})
	}

	// TODO: not a fan of the subrow id in this
	fn page_for_row(
		&self,
		row_id: u32,
		subrow_id: u16,
		language: Option<Language>,
	) -> Result<Arc<exd::ExcelData>> {
		let header = self.header()?;

		let start_id = header
			.pages()
			.iter()
			.find(|page| page.start_id() <= row_id && page.start_id() + page.row_count() > row_id)
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Row {
					row: row_id,
					subrow: subrow_id,
					sheet: self.name().into(),
				})
			})?
			.start_id();

		self.page(start_id, language)
	}

	pub(super) fn page(
		&self,
		start_id: u32,
		language: Option<Language>,
	) -> Result<Arc<exd::ExcelData>> {
		let language = self.resolve_language(language)?;

		// Try to read in the page for the requested (sub)row.
		self.cache
			.pages
			.try_get_or_insert((start_id, language), || {
				let path = path::exd(&self.name(), start_id, language);
				self.ironworks.file(&path)
			})
	}

	fn resolve_language(&self, language: Option<Language>) -> Result<Language> {
		let header = self.header()?;

		// Get the language to load, or NONE if the language is not supported by this sheet.
		// TODO: Should an explicit language request fail hard on miss?
		let requested_language = language.unwrap_or(self.default_language);
		[requested_language, Language::None]
			.into_iter()
			.find(|&language| header.languages().contains(&language.into()))
			// TODO: Should this be Invalid or NotFound?
			// TODO: Should we have an explicit ErrorValue for language?
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Other(format!(
					"language {requested_language:?}"
				)))
			})
	}
}
