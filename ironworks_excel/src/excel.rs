use std::{fmt::Debug, rc::Rc};

use crate::{error::Error, header::ExcelHeader, list::ExcelList, sheet::RawExcelSheet};

pub type ResourceResult<T> = Result<T, anyhow::Error>;

// TODO: Consider if this should just return Result<Vec<u8>, _>
pub trait ExcelResource: Debug {
	fn list(&self) -> ResourceResult<ExcelList>;
	fn header(&self, sheet_name: &str) -> ResourceResult<ExcelHeader>;
	// fn page(&self) -> ResourceResult<ExcelPage>;
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
		// TODO: who owns caching this?
		let list = self.resource.list()?;

		if !list.has_sheet(sheet_name) {
			return Err(Error::NotFound(format!("Sheet \"{}\"", sheet_name)));
		}

		Ok(RawExcelSheet::new(sheet_name, self.resource.clone()))
	}
}

// maybe like rawsheet is a self-impl of sheet which also uses sheetreader?
