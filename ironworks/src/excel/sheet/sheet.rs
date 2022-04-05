use std::io::Cursor;

use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::Resource,
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
};

use super::{
	header::{Header, SheetKind},
	page::Page,
	row::{RowHeader, SubrowHeader},
	row_options::RowOptions,
};

const LANGUAGE_NONE: u8 = 0;

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet<'r, R> {
	sheet: String,

	resource: &'r R,

	header: OptionCache<Header>,
	pages: HashMapCache<(u32, u8), Page>,
}

impl<'r, R: Resource> Sheet<'r, R> {
	pub(crate) fn new(sheet: String, resource: &'r R) -> Self {
		Self {
			sheet,

			resource,

			header: Default::default(),
			pages: Default::default(),
		}
	}

	/// Create a row options builder for this sheet.
	pub fn with(&'r self) -> RowOptions<'r, R> {
		RowOptions::new(self)
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<()> {
		self.row_with_options(row_id, &Default::default())
	}

	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<()> {
		self.subrow_with_options(row_id, subrow_id, &Default::default())
	}

	pub(super) fn row_with_options(&self, row_id: u32, options: &RowOptions<'r, R>) -> Result<()> {
		self.subrow_with_options(row_id, 0, options)
	}

	pub(super) fn subrow_with_options(
		&self,
		row_id: u32,
		subrow_id: u16,
		options: &RowOptions<'r, R>,
	) -> Result<()> {
		let header = self.header.try_get_or_insert(|| {
			let mut reader = self.resource.header(&self.sheet)?;
			Header::read(&mut reader).map_err(|error| Error::Resource(error.into()))
		})?;

		let row_not_found = || {
			Error::NotFound(ErrorValue::Row {
				row: row_id,
				subrow: subrow_id,
				sheet: self.sheet.clone(),
			})
		};

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if header.kind != SheetKind::Subrows && subrow_id > 0 {
			return Err(row_not_found());
		}

		// Get the language to load, or NONE if the language is not supported by this sheet.
		// TODO: Should an explicit language request fail hard on miss?
		// TODO: Sheet-wide (and global) language defaults?
		let requested_language = options.language.unwrap_or(LANGUAGE_NONE);
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
			let mut reader = self.resource.page(&self.sheet, start_id, language)?;
			Page::read(&mut reader).map_err(|error| Error::Resource(error.into()))
		})?;

		// Find the row definition in the page. If it's missing, there's something
		// wrong with the provided resource.
		let row_definition = page.rows.iter().find(|row| row.id == row_id).ok_or_else(|| {
			Error::Resource(format!("{} sheet header indicates row ID {row_id} exists in page {start_id}:{language}, but page header does not define it.", self.sheet).into())
		})?;

		// Read & sanity check the row header
		let mut cursor = Cursor::new(&page.data);
		cursor.set_position(row_definition.offset.into());
		let row_header =
			RowHeader::read(&mut cursor).map_err(|error| Error::Resource(error.into()))?;

		if subrow_id >= row_header.row_count {
			return Err(row_not_found());
		}

		// Slice the data for the requested (sub) row.
		let mut offset: usize = cursor.position().try_into().unwrap();
		if header.kind == SheetKind::Subrows {
			offset += usize::from(
				subrow_id * (header.row_size + SubrowHeader::SIZE) + SubrowHeader::SIZE,
			);
		}

		let mut length: usize = header.row_size.try_into().unwrap();
		if header.kind != SheetKind::Subrows {
			length += usize::try_from(row_header.data_size).unwrap();
		}

		let data = &page.data[offset..offset + length];

		println!("new rdat: {data:?}");

		Ok(())
	}
}
