use std::{fmt::Debug, sync::Arc};

use crate::{
	error::{Error, ErrorValue, Result},
	excel::{borrowed::Borrowed, mapper::Mapper, metadata::SheetMetadata, row::Row},
	file::{exd, exh},
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
	Ironworks,
};

use super::row_options::RowOptions;

// TODO: Where should this go? It's also effectively used by the main Excel struct.
const LANGUAGE_NONE: u8 = 0;

// TODO: how much should be in this? Arguably the mapper &co might be relevant given that the mapper is required to fill the caches, etc.
/// Data cache for raw values, decoupled from mapping/metadata concerns.
#[derive(Debug, Default)]
pub struct SheetCache {
	header: OptionCache<exh::ExcelHeader>,
	pages: HashMapCache<(u32, u8), exd::ExcelData>,
}

/// A sheet within an Excel database.
pub struct Sheet<'i, S> {
	sheet_metadata: S,
	default_language: u8,

	ironworks: Borrowed<'i, Ironworks>,
	mapper: &'i dyn Mapper,

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
		default_language: u8,
		ironworks: Borrowed<'i, Ironworks>,
		mapper: &'i dyn Mapper,
		cache: Arc<SheetCache>,
	) -> Self {
		Self {
			sheet_metadata,
			default_language,

			ironworks,
			mapper,

			cache,
		}
	}

	/// Get the kind of this sheet.
	pub fn kind(&self) -> Result<exh::SheetKind> {
		let kind = self.header()?.kind();
		Ok(kind)
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

	/// Build an iterator over the rows in this sheet.
	pub fn iter(&'i self) -> SheetIterator<'i, S> {
		SheetIterator::new(self)
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
			sheet: self.sheet_metadata.name().into(),
		};

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if header.kind() != exh::SheetKind::Subrows && subrow_id > 0 {
			return Err(Error::NotFound(row_error_value()));
		}

		// Try to read in the page for the requested (sub)row.
		let page = self.page(row_id, subrow_id, options.language)?;

		let data = match header.kind() {
			exh::SheetKind::Subrows => page.subrow_data(row_id, subrow_id),
			_ => page.row_data(row_id),
		}?;
		let row = Row::new(row_id, subrow_id, header, data.to_vec());

		self.sheet_metadata
			.populate_row(row)
			.map_err(|error| Error::Invalid(row_error_value(), error.to_string()))
	}

	fn header(&self) -> Result<Arc<exh::ExcelHeader>> {
		self.cache.header.try_get_or_insert(|| {
			let path = self.mapper.exh(&self.sheet_metadata.name());
			self.ironworks.file(&path)
		})
	}

	// TODO: not a fan of the subrow id in this
	fn page(
		&self,
		row_id: u32,
		subrow_id: u16,
		language: Option<u8>,
	) -> Result<Arc<exd::ExcelData>> {
		let header = self.header()?;

		// Get the language to load, or NONE if the language is not supported by this sheet.
		// TODO: Should an explicit language request fail hard on miss?
		let requested_language = language.unwrap_or(self.default_language);
		let language = *header
			.languages()
			.get(&requested_language)
			.or_else(|| header.languages().get(&LANGUAGE_NONE))
			// TODO: Should this be Invalid or NotFound?
			// TODO: Should we have an explicit ErrorValue for language?
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Other(format!("language {requested_language}")))
			})?;

		let start_id = header
			.pages()
			.iter()
			.find(|page| page.start_id() <= row_id && page.start_id() + page.row_count() > row_id)
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Row {
					row: row_id,
					subrow: subrow_id,
					sheet: self.sheet_metadata.name().into(),
				})
			})?
			.start_id();

		// Try to read in the page for the requested (sub)row.
		self.cache
			.pages
			.try_get_or_insert((start_id, language), || {
				let path = self
					.mapper
					.exd(&self.sheet_metadata.name(), start_id, language);
				self.ironworks.file(&path)
			})
	}
}

/// An iterator that iterates over the rows of an excel sheet.
#[derive(Debug)]
pub struct SheetIterator<'i, S> {
	sheet: &'i Sheet<'i, S>,

	row_id: u32,
	subrow_id: u16,

	subrow_count: Option<u16>,
}

impl<'i, S: SheetMetadata> SheetIterator<'i, S> {
	fn new(sheet: &'i Sheet<S>) -> Self {
		SheetIterator {
			sheet,
			row_id: 0,
			subrow_id: 0,
			subrow_count: None,
		}
	}
}

impl<S: SheetMetadata> Iterator for SheetIterator<'_, S> {
	type Item = S::Row;

	fn next(&mut self) -> Option<Self::Item> {
		// TODO: both the .page and .subrow calls should have some means to utilise an iter-wide lang override

		let subrow_count = match self.subrow_count {
			Some(v) => v,
			None => {
				let page = self.sheet.page(self.row_id, self.subrow_id, None).ok()?;
				let subrow_count = page.subrow_count(self.row_id).ok()?;
				*self.subrow_count.insert(subrow_count)
			}
		};

		if self.subrow_id >= subrow_count {
			self.row_id += 1;
			self.subrow_id = 0;
			self.subrow_count = None;
		}

		let row = self.sheet.subrow(self.row_id, self.subrow_id).ok()?;

		self.subrow_id += 1;

		Some(row)
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		use exh::SheetKind as K;
		match self.sheet.header().ok()?.kind() {
			// Subrows have to be done the manual way, as there's no way to know the subrow count without reading the relevant .exd page.
			K::Subrows => {
				for _i in 0..n {
					self.next()?;
				}
			}
			_ => self.row_id = n.try_into().unwrap(),
		}

		self.next()
	}
}
