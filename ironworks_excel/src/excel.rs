use std::{fmt::Debug, rc::Rc};

use crate::{
	error::{Error, Result},
	list::ExcelList,
	sheet::RawExcelSheet,
};

pub type ResourceResult<T> = std::result::Result<T, anyhow::Error>;

pub trait ExcelResource: Debug {
	fn list(&self) -> ResourceResult<Vec<u8>>;
	fn header(&self, sheet_name: &str) -> ResourceResult<Vec<u8>>;
	fn page(&self, sheet_name: &str, start_id: u32, language_id: u8) -> ResourceResult<Vec<u8>>;
}

pub struct Excel<'a> {
	resource: Rc<dyn ExcelResource + 'a>,
}

impl<'a> Excel<'a> {
	pub fn new(resource: impl ExcelResource + 'a) -> Self {
		Self {
			resource: Rc::new(resource),
		}
	}

	pub fn get_raw_sheet(&self, sheet_name: &str) -> Result<RawExcelSheet> {
		let list = self.get_list()?;

		if !list.has_sheet(sheet_name) {
			return Err(Error::NotFound(format!("Sheet \"{}\"", sheet_name)));
		}

		// todo: possibly should cache the raw sheets
		Ok(RawExcelSheet::new(sheet_name, self.resource.clone()))
	}

	fn get_list(&self) -> Result<ExcelList> {
		// todo: cache
		let bytes = self.resource.list()?;
		let list = ExcelList::from_bytes(bytes)?;
		Ok(list)
	}
}

// maybe like rawsheet is a self-impl of sheet which also uses sheetreader?
