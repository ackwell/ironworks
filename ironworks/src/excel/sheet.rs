use std::{
	collections::HashMap,
	io::{Cursor, Seek},
	sync::{Arc, OnceLock, RwLock},
};

use binrw::BinRead;
use derivative::Derivative;
use num_enum::FromPrimitive;

use crate::{
	error::{Error, ErrorValue, Result},
	file::{exd, exh},
	ironworks::Ironworks,
};

use super::{iterator::SheetIterator, language::Language, metadata::SheetMetadata, path, row::Row};

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

		let row_definition = row_definition(&page, row_id)?;

		let mut cursor = Cursor::new(&page.data);
		cursor.set_position(u64::from(row_definition.offset));
		let row_header = exd::RowHeader::read(&mut cursor)?;

		let row_data_pos = usize::try_from(cursor.position()).unwrap();
		let row_data_size = usize::try_from(row_header.size).unwrap();

		let row_data = &page.data[row_data_pos..row_data_pos + row_data_size];
		let (field_buffer, string_buffer) =
			subrow_buffers(&sheet_header, &row_header, row_data, row_id, subrow_id)?;

		// TODO: This means I'm cloning the entire row byte array each time, even if someone's asking for 2 fields. Perhaps consider using a "row reader" that operates on a temporary lifetime with the byte slice, and only to_vec the data in a concrete Row for raw reading?
		let row = Row::new(
			row_id,
			subrow_id,
			sheet_header,
			field_buffer.to_vec(),
			string_buffer.to_vec(),
		);

		self.metadata
			.populate_row(row)
			.map_err(|error| Error::Invalid(row_error_value(), error.to_string()))
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

	pub(super) fn page(&self, start_id: u32, language: Language) -> Result<Arc<exd::ExcelData>> {
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
		let data = Arc::new(self.ironworks.file::<exd::ExcelData>(&path)?);

		pages_mut.insert(key, data.clone());

		Ok(data)
	}

	pub(super) fn resolve_language(&self, language: Language) -> Result<Language> {
		let header = self.header()?;

		// Get the language to load, or NONE if the language is not supported by this sheet.
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
	type Item = S::Row;
	type IntoIter = SheetIterator<S>;

	fn into_iter(self) -> Self::IntoIter {
		SheetIterator::new(self)
	}
}

// TODO: this is only pub for use by the current (horrid) iterator.
pub(super) fn row_definition(page: &exd::ExcelData, row_id: u32) -> Result<&exd::RowDefinition> {
	// Most pages are contiguous IDs - check if this assumption holds in this
	// case, and fast track if it does.
	let first_row_id = page.rows.get(0).map_or(0, |row| row.id);
	if let Some(index) = row_id.checked_sub(first_row_id) {
		let index_usize = usize::try_from(index).unwrap();
		if index_usize < page.rows.len() && page.rows[index_usize].id == row_id {
			return Ok(&page.rows[index_usize]);
		}
	}

	// Otherwise, fall back to a naive scan.
	page.rows
		.iter()
		.find(|row| row.id == row_id)
		.ok_or(Error::NotFound(ErrorValue::Row {
			row: row_id,
			subrow: 0,
			sheet: None,
		}))
}

fn subrow_buffers<'a>(
	sheet_header: &exh::ExcelHeader,
	row_header: &exd::RowHeader,
	row_data: &'a [u8],
	row_id: u32,
	subrow_id: u16,
) -> Result<(&'a [u8], &'a [u8])> {
	// For non-subrow sheets, there should be a single set of fields at offset 0,
	// followed by the string buffer.
	if sheet_header.kind != exh::SheetKind::Subrows {
		return Ok(row_data.split_at(sheet_header.row_size.into()));
	}

	let mut maybe_fields_pos = None;
	let mut cursor = Cursor::new(row_data);
	for _ in 0..row_header.count {
		let subrow_header = exd::SubrowHeader::read(&mut cursor)?;
		if subrow_header.id == subrow_id {
			maybe_fields_pos = Some(usize::try_from(cursor.position()).unwrap());
			break;
		}
		cursor.seek_relative(sheet_header.row_size.into())?;
	}

	let Some(fields_pos) = maybe_fields_pos else {
		// TODO: this better
		return Err(Error::NotFound(ErrorValue::Row {
			row: row_id,
			subrow: subrow_id,
			sheet: None,
		}));
	};

	// String buffer sits after all subrow field data.
	let strings_pos = usize::from(row_header.count)
		* (usize::from(sheet_header.row_size) + exd::SubrowHeader::SIZE);

	Ok((
		&row_data[fields_pos..fields_pos + usize::from(sheet_header.row_size)],
		&row_data[strings_pos..],
	))
}

/// Data cache for raw values, decoupled from mapping/metadata concerns.
#[derive(Default)]
pub struct SheetCache {
	header: OnceLock<Arc<exh::ExcelHeader>>,
	pages: RwLock<HashMap<(u32, Language), Arc<exd::ExcelData>>>,
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
