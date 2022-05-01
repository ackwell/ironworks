use std::{fmt::Debug, sync::Arc};

use crate::{
	error::{Error, ErrorValue, Result},
	excel::{mapper::Mapper, metadata::SheetMetadata, row::Row},
	file::{exd, exh},
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
	Ironworks,
};

use super::row_options::RowOptions;

// TODO: Where should this go? It's also effectively used by the main Excel struct.
const LANGUAGE_NONE: u8 = 0;

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
pub struct Sheet<'i, S> {
	sheet_metadata: S,
	default_language: u8,

	ironworks: &'i Ironworks,
	mapper: &'i dyn Mapper,

	header: OptionCache<exh::ExcelHeader>,
	pages: HashMapCache<(u32, u8), exd::ExcelData>,
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
		default_language: u8,
		ironworks: &'i Ironworks,
		mapper: &'i dyn Mapper,
	) -> Self {
		Self {
			sheet_metadata,
			default_language,

			ironworks,
			mapper,

			header: Default::default(),
			pages: Default::default(),
		}
	}

	/// Fetch metadata for all columns in this sheet.
	pub fn columns(&self) -> Result<Vec<exh::ColumnDefinition>> {
		let header = self.header()?;
		Ok(header.columns.clone())
	}

	// TODO: name. row_with? "with" refers to construction, sorta.
	/// Create a row options builder for this sheet.
	pub fn with(&'i self) -> RowOptions<'i, S> {
		RowOptions::new(self)
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<S::Row> {
		self.row_with_options(row_id, &Default::default())
	}

	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<S::Row> {
		self.subrow_with_options(row_id, subrow_id, &Default::default())
	}

	pub(super) fn row_with_options(
		&self,
		row_id: u32,
		options: &RowOptions<'i, S>,
	) -> Result<S::Row> {
		self.subrow_with_options(row_id, 0, options)
	}

	// TODO: this fn is absurdly long. split it up.
	pub(super) fn subrow_with_options(
		&self,
		row_id: u32,
		subrow_id: u16,
		options: &RowOptions<'i, S>,
	) -> Result<S::Row> {
		let header = self.header()?;

		let row_error_value = || ErrorValue::Row {
			row: row_id,
			subrow: subrow_id,
			sheet: self.sheet_metadata.name(),
		};
		let row_not_found = || Error::NotFound(row_error_value());

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if header.kind != exh::SheetKind::Subrows && subrow_id > 0 {
			return Err(row_not_found());
		}

		// Get the language to load, or NONE if the language is not supported by this sheet.
		// TODO: Should an explicit language request fail hard on miss?
		let requested_language = options.language.unwrap_or(self.default_language);
		let language = *header
			.languages
			.get(&requested_language)
			.or_else(|| header.languages.get(&LANGUAGE_NONE))
			// TODO: Should this be Invalid or NotFound?
			// TODO: Should we have an explicit ErrorValue for language?
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Other(format!("language {requested_language}")))
			})?;

		// Try to read in the page for the requested (sub)row.
		let start_id = header
			.pages
			.iter()
			.find(|page| page.start_id <= row_id && page.start_id + page.row_count > row_id)
			.ok_or_else(row_not_found)?
			.start_id;

		let page = self.pages.try_get_or_insert((start_id, language), || {
			let path = self
				.mapper
				.exd(&self.sheet_metadata.name(), start_id, language);
			self.ironworks.file(&path)
		})?;

		let data = match header.kind {
			exh::SheetKind::Subrows => page.subrow_data(row_id, subrow_id),
			_ => page.row_data(row_id),
		}?;
		let row = Row::new(row_id, subrow_id, header, data.to_vec());

		self.sheet_metadata
			.populate_row(row)
			.map_err(|error| Error::Invalid(row_error_value(), error.to_string()))
	}

	fn header(&self) -> Result<Arc<exh::ExcelHeader>> {
		self.header.try_get_or_insert(|| {
			let path = self.mapper.exh(&self.sheet_metadata.name());
			self.ironworks.file(&path)
		})
	}
}
