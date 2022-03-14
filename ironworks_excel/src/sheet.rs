use std::rc::Rc;

use crate::{
	error::{Error, Result},
	excel::ExcelResource,
	header::ExcelHeader,
	page::ExcelPage,
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

		// ---

		println!("{:#?}", page_definition);

		// read in page struct thing
		let page = self.get_page(page_definition.start_id)?;

		println!("{:#?}", page);

		// get row in page
		// is this just a byte slice? what about the extradata shit for strings &c?

		// build row reader
		// is row reader going to pull directly out of page, or?

		Ok(RowReader {})
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

// TODO put this somewhere sensible
// TODO this is basically a raw row - standardise naming with the raw sheet. do we have a sheetreader and rowreader, or rawsheet and rawrow, or...
#[derive(Debug)]
pub struct RowReader {}
