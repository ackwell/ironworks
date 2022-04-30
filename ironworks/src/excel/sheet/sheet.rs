use std::{
	fmt::Debug,
	io::{Cursor, Seek, SeekFrom},
	sync::Arc,
};

use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::{mapper::Mapper, metadata::SheetMetadata},
	file,
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
	Ironworks,
};

use super::{
	page::Page,
	row::{Row, RowHeader, SubrowHeader},
	row_options::RowOptions,
};

// TODO: Where should this go? It's also effectively used by the main Excel struct.
const LANGUAGE_NONE: u8 = 0;

/// Metadata about a column within a sheet.
#[derive(Debug)]
pub struct Column {
	index: usize,
	offset: u16,
	kind: file::exh::ColumnKind,
}

impl Column {
	/// Index of this column within the sheet header.
	pub fn index(&self) -> usize {
		self.index
	}

	/// Offset of this column's data within row binary data.
	pub fn offset(&self) -> u16 {
		self.offset
	}

	/// Kind of data held in this column.
	pub fn kind(&self) -> file::exh::ColumnKind {
		self.kind
	}
}

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
pub struct Sheet<'i, S> {
	sheet_metadata: S,
	default_language: u8,

	ironworks: &'i Ironworks,
	mapper: &'i dyn Mapper,

	header: OptionCache<file::exh::Exh>,
	pages: HashMapCache<(u32, u8), Page>,
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
	pub fn columns(&self) -> Result<Vec<Column>> {
		let header = self.header()?;
		let columns = header
			.columns
			.iter()
			.enumerate()
			.map(|(index, definition)| Column {
				index,
				offset: definition.offset,
				kind: definition.kind,
			})
			.collect::<Vec<_>>();

		Ok(columns)
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
		if header.kind != file::exh::SheetKind::Subrows && subrow_id > 0 {
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

		// Find the row definition in the page. If it's missing, there's something
		// wrong with the provided resource.
		let row_definition = page.rows.iter().find(|row| row.id == row_id).ok_or_else(|| {
			Error::Resource(format!("{} sheet header indicates row ID {row_id} exists in page {start_id}:{language}, but page header does not define it.", self.sheet_metadata.name()).into())
		})?;

		// Read & sanity check the row header
		let mut cursor = Cursor::new(&page.data);
		cursor.set_position(row_definition.offset.into());
		let row_header =
			RowHeader::read(&mut cursor).map_err(|error| Error::Resource(error.into()))?;

		if subrow_id >= row_header.row_count {
			return Err(row_not_found());
		}

		// If this is a subrow sheet, jump to the start of the requested subrow and
		// double check the ID matches.
		let mut resource_subrow_id = 0u16;
		if header.kind == file::exh::SheetKind::Subrows {
			cursor
				.seek(SeekFrom::Current(
					(subrow_id * (SubrowHeader::SIZE + header.row_size)).into(),
				))
				.map_err(|error| Error::Resource(error.into()))?;
			let subrow_header =
				SubrowHeader::read(&mut cursor).map_err(|error| Error::Resource(error.into()))?;

			if subrow_header.id != subrow_id {
				return Err(Error::Resource(
					format!(
						"Data for subrow {subrow_id} exists, but self-reports as subrow {}",
						subrow_header.id
					)
					.into(),
				));
			}

			resource_subrow_id = subrow_header.id;
		}

		// Slice the data for the requested (sub) row.
		let offset: usize = cursor.position().try_into().unwrap();
		let mut length: usize = header.row_size.try_into().unwrap();
		if header.kind != file::exh::SheetKind::Subrows {
			length += usize::try_from(row_header.data_size).unwrap();
		}

		let data = &page.data[offset..offset + length];
		let row = Row::new(row_definition.id, resource_subrow_id, header, data.to_vec());

		self.sheet_metadata
			.populate_row(row)
			.map_err(|error| Error::Invalid(row_error_value(), error.to_string()))
	}

	fn header(&self) -> Result<Arc<file::exh::Exh>> {
		self.header.try_get_or_insert(|| {
			let path = self.mapper.exh(&self.sheet_metadata.name());
			self.ironworks.file(&path)
		})
	}
}
