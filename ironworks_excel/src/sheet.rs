use std::{io::Cursor, rc::Rc};

use binrw::BinRead;

use crate::{
	error::{Error, Result},
	excel::ExcelResource,
	header::{Header, SheetKind},
	page::Page,
	row::{RowHeader, RowReader, SubrowHeader},
};

const LANGUAGE_NONE: u8 = 0;

pub struct SheetOptions {
	pub default_language: u8,
}

// TODO: should this be in row?
pub struct RowOptions {
	pub language: Option<u8>,
}

impl RowOptions {
	pub fn new() -> Self {
		Self { language: None }
	}

	pub fn language(&mut self, value: impl Into<u8>) -> &mut Self {
		self.language = Some(value.into());
		self
	}
}

impl Default for RowOptions {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug)]
pub struct SheetReader<'a> {
	sheet_name: String,
	default_language: u8,

	resource: Rc<dyn ExcelResource + 'a>,
}

impl<'a> SheetReader<'a> {
	// pub(crate)?
	pub fn with_options(
		sheet_name: &str,
		resource: Rc<dyn ExcelResource + 'a>,
		options: SheetOptions,
	) -> Self {
		Self {
			sheet_name: sheet_name.into(),
			default_language: options.default_language,
			resource,
		}
	}

	// todo iterable rows?

	#[inline]
	pub fn row(&self, row_id: u32) -> Result<RowReader> {
		self.subrow(row_id, 0)
	}

	#[inline]
	pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Result<RowReader> {
		self.subrow_with_options(row_id, subrow_id, &RowOptions::new())
	}

	#[inline]
	pub fn row_with_options(&self, row_id: u32, options: &RowOptions) -> Result<RowReader> {
		self.subrow_with_options(row_id, 0, options)
	}

	// TODO: think about the api a bit. it might be nice to do something like
	// sheet.with_options().language(...).get_row(N)
	// "with options" is a bit weird there, think?
	pub fn subrow_with_options(
		&self,
		row_id: u32,
		subrow_id: u16,
		options: &RowOptions,
	) -> Result<RowReader> {
		let header = self.header()?;

		// Only subrow sheets support a subrow > 0, fail early if possible.
		if header.kind != SheetKind::Subrows && subrow_id > 0 {
			// TODO: Improve error message.
			return Err(Error::NotFound(format!("Subrow ID \"{}\"", subrow_id)));
		}

		// Get the language for strings, falling back to none if the sheet does not
		// support it.
		// TODO: do we want an explicit language request in row options to fail hard without defaulting?
		let requested_language = options.language.unwrap_or(self.default_language);
		let language = header
			.languages
			.get(&requested_language)
			.or_else(|| header.languages.get(&LANGUAGE_NONE))
			// TODO: Not conviced this should be NotFound.
			.ok_or_else(|| Error::NotFound(format!("Language \"{}\"", requested_language)))?;

		// Find the page definition for the requested row, if any.
		let page_definition = header
			.pages
			.iter()
			.find(|page| page.start_id <= row_id && page.start_id + page.row_count > row_id)
			.ok_or_else(|| Error::NotFound(format!("Row ID \"{}\"", row_id)))?;

		let page = self.page(page_definition.start_id, *language)?;

		// Find the row definition for the requested row. A failure here implies
		// corrupt resources.
		let row_definition = page
			.header
			.rows
			.iter()
			.find(|row| row.row_id == row_id)
			.ok_or_else(|| {
				Error::InvalidResource(format!(
					"Row ID {} found in sheet header, but provided page does not define it.",
					row_id,
				))
			})?;

		// Read the row's header.
		let mut cursor = Cursor::new(&page.data);
		cursor.set_position(row_definition.offset.into());
		let row_header = RowHeader::read(&mut cursor).map_err(|error| {
			Error::InvalidResource(format!(
				"Failed to read header of row {}: {}",
				row_id, error
			))
		})?;

		// Make sure the requested subrow ID is available from this row.
		if subrow_id >= row_header.row_count {
			return Err(Error::NotFound(format!("Subrow ID \"{}\"", subrow_id)));
		}

		// Slice the page data for just the requested row.
		let mut offset = cursor.position() as usize;
		if header.kind == SheetKind::Subrows {
			offset += subrow_id as usize * (header.row_size as usize + SubrowHeader::SIZE)
				+ SubrowHeader::SIZE;
		}

		let mut length = header.row_size as usize;
		if header.kind != SheetKind::Subrows {
			length += row_header.data_size as usize
		}

		let data = &page.data[offset..offset + length];

		Ok(RowReader::new(row_id, subrow_id, header, data))
	}

	fn header(&self) -> Result<Rc<Header>> {
		// todo: cache
		let bytes = self.resource.header(&self.sheet_name)?;
		let header = Header::from_bytes(bytes)?;
		Ok(Rc::new(header))
	}

	fn page(&self, start_id: u32, language: u8) -> Result<Page> {
		// TODO: cache
		let bytes = self.resource.page(&self.sheet_name, start_id, language)?;
		let page = Page::from_bytes(bytes)?;
		Ok(page)
	}
}
