use std::rc::Rc;

use crate::{error::Result, header::ExcelHeader, ExcelResource};

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

	pub fn get_row(&self, row_id: u16) -> Result<RowReader> {
		self.get_subrow(row_id, 0)
	}

	pub fn get_subrow(&self, row_id: u16, subrow_id: u16) -> Result<RowReader> {
		let header = self.get_header()?;

		// get page definition
		// header

		// get row in page
		// is this just a byte slice? what about the extradata shit for strings &c?

		// build row reader
		// is row reader going to pull directly out of page, or?

		Ok(RowReader {})
	}

	fn get_header(&self) -> Result<ExcelHeader> {
		// todo: cache
		let header_bytes = self.resource.header(&self.sheet_name)?;
		let header = ExcelHeader::from_bytes(&header_bytes)?;
		Ok(header)
	}
}

// TODO put this somewhere sensible
// TODO this is basically a raw row - standardise naming with the raw sheet. do we have a sheetreader and rowreader, or rawsheet and rawrow, or...
#[derive(Debug)]
pub struct RowReader {}
