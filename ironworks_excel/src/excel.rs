use std::{fmt::Debug, rc::Rc};

use crate::{error::Error, list::ExcelList, sheet::RawExcelSheet};

pub type ResourceResult<T> = Result<T, anyhow::Error>;

pub trait ExcelResource: Debug {
	fn list(&self) -> ResourceResult<Vec<u8>>;
	fn header(&self, sheet_name: &str) -> ResourceResult<Vec<u8>>;
	// fn page(&self) -> ResourceResult<Vec<u8>>;
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

	pub fn get_raw_sheet(&self, sheet_name: &str) -> Result<RawExcelSheet, Error> {
		let list = self.get_list()?;

		if !list.has_sheet(sheet_name) {
			return Err(Error::NotFound(format!("Sheet \"{}\"", sheet_name)));
		}

		// todo: possibly should cache the raw sheets
		Ok(RawExcelSheet::new(sheet_name, self.resource.clone()))
	}

	fn get_list(&self) -> Result<ExcelList, Error> {
		// todo: cache
		let bytes = self.resource.list()?;
		let list = ExcelList::from_bytes(&bytes)?;
		Ok(list)
	}
}

// maybe like rawsheet is a self-impl of sheet which also uses sheetreader?
