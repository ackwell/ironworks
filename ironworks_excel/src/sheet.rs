use std::{io::Cursor, rc::Rc};

use binrw::BinRead;

use crate::{
	error::{Error, Result},
	excel::ExcelResource,
	header::ExcelHeader,
	page::ExcelPage,
	row::{ExcelRowHeader, RowReader},
};

pub struct SheetOptions {
	pub default_language: u8,
}

// TODO should this be ExcelRawSheet?
#[derive(Debug)]
pub struct RawExcelSheet<'a> {
	sheet_name: String,
	default_language: u8,

	resource: Rc<dyn ExcelResource + 'a>,
}

impl<'a> RawExcelSheet<'a> {
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
	// todo: lang?

	pub fn get_row(&self, row_id: u32) -> Result<RowReader> {
		self.get_subrow(row_id, 0)
	}

	pub fn get_subrow(&self, row_id: u32, subrow_id: u32) -> Result<RowReader> {
		let header = self.get_header()?;

		// Find the page definition for the requested row, if any.
		let page_definition = header
			.pages
			.iter()
			.find(|page| page.start_id <= row_id && page.start_id + page.row_count > row_id)
			.ok_or_else(|| Error::NotFound(format!("Row ID \"{}\"", row_id)))?;

		let page = self.get_page(page_definition.start_id)?;

		// Find the row definition for the requested row. A failure here implies
		// corrupt resources.
		let row_definition = page
			.header
			.rows
			.iter()
			.find(|row| row.row_id == row_id)
			.expect("Requested row ID is not defined by the provided page.");

		// Read the row's header.
		// TODO: handle subrows + validation
		let mut cursor = Cursor::new(&page.data);
		cursor.set_position(row_definition.offset.into());
		let row_header = ExcelRowHeader::read(&mut cursor).unwrap();

		// Slice the page data for just the requested row.
		let offset = cursor.position() as usize;
		// TODO: Check data_length behavior on a subrow sheet.
		let length = header.row_size as usize + row_header.data_size as usize;
		let data = &page.data[offset..offset + length];

		let row_reader = RowReader::new(header, data);

		Ok(row_reader)
	}

	fn get_header(&self) -> Result<Rc<ExcelHeader>> {
		// todo: cache
		let bytes = self.resource.header(&self.sheet_name)?;
		let header = ExcelHeader::from_bytes(bytes)?;
		Ok(Rc::new(header))
	}

	fn get_page(&self, start_id: u32) -> Result<ExcelPage> {
		// TODO: this _needs_ to handle language
		// TODO: cache
		let bytes = self
			.resource
			.page(&self.sheet_name, start_id, self.default_language)?;
		let page = ExcelPage::from_bytes(bytes)?;
		Ok(page)
	}
}
