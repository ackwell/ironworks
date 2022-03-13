use crate::{error::Error, header::ExcelHeader, list::ExcelList, sheet::RawExcelSheet};

pub type ResourceResult<T> = Result<T, anyhow::Error>;

pub trait ExcelResource {
	fn list(&self) -> ResourceResult<ExcelList>;
	fn header(&self, sheet_name: &str) -> ResourceResult<ExcelHeader>;
	// fn page(&self) -> ResourceResult<ExcelPage>;
}

pub struct Excel<'a> {
	resource: Box<dyn ExcelResource + 'a>,
}

impl<'a> Excel<'a> {
	pub fn new(resource: impl ExcelResource + 'a) -> Self {
		Self {
			resource: Box::new(resource),
		}
	}

	pub fn get_raw_sheet(&self, sheet_name: &str) -> Result<RawExcelSheet, Error> {
		// TODO: who owns caching this?
		let list = self.resource.list()?;

		if !list.has_sheet(sheet_name) {
			return Err(Error::NotFound(format!("Sheet \"{}\"", sheet_name)));
		}

		// TODO: we're going to need to retrieve pages as-needed from the sheet, so maybe we should pass the resource directly to the sheet?
		let header = self.resource.header(sheet_name)?;

		Ok(RawExcelSheet { header })
	}
}
