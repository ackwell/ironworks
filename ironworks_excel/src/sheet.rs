use std::{io::Cursor, rc::Rc};

use binrw::BinRead;

use crate::{
	error::{Error, Result},
	excel::ExcelResource,
	header::ExcelHeader,
	page::ExcelPage,
	row::{ExcelRowHeader, RowReader},
};

// TODO should this be ExcelRawSheet?
#[derive(Debug)]
pub struct RawExcelSheet<'a> {
	sheet_name: String,

	resource: Rc<dyn ExcelResource + 'a>,
}

impl<'a> RawExcelSheet<'a> {
	pub fn new(sheet_name: &str, resource: Rc<dyn ExcelResource + 'a>) -> Self {
		Self {
			sheet_name: sheet_name.into(),
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
		let length = header.data_offset as usize + row_header.data_size as usize;
		let data = &page.data[offset..offset + length];

		let row_reader = RowReader::new(&header.columns, data);

		Ok(row_reader)
	}

	fn get_header(&self) -> Result<ExcelHeader> {
		// todo: cache
		let bytes = self.resource.header(&self.sheet_name)?;
		let header = ExcelHeader::from_bytes(bytes)?;
		Ok(header)
	}

	fn get_page(&self, start_id: u32) -> Result<ExcelPage> {
		// TODO: this _needs_ to handle language
		// TODO: cache
		let bytes = self.resource.page(&self.sheet_name, start_id)?;
		let page = ExcelPage::from_bytes(bytes)?;
		Ok(page)
	}
}
