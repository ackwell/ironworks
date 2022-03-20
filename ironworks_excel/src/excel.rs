use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
	error::{Error, Result},
	list::List,
	sheet::{SheetOptions, SheetReader},
};

pub type ResourceResult<T> = std::result::Result<T, anyhow::Error>;

pub trait ExcelResource: Debug {
	fn list(&self) -> ResourceResult<Vec<u8>>;
	fn header(&self, sheet_name: &str) -> ResourceResult<Vec<u8>>;
	fn page(&self, sheet_name: &str, start_id: u32, language_id: u8) -> ResourceResult<Vec<u8>>;
}

pub struct ExcelOptions {
	pub default_language: u8,
}

impl ExcelOptions {
	fn new() -> Self {
		Self {
			default_language: 0,
		}
	}
}

pub struct Excel<'a> {
	default_language: u8,
	resource: Rc<dyn ExcelResource + 'a>,

	list: RefCell<Option<Rc<List>>>,
}

impl<'a> Excel<'a> {
	#[inline]
	pub fn new(resource: impl ExcelResource + 'a) -> Self {
		Self::with_options(resource, ExcelOptions::new())
	}

	pub fn with_options(resource: impl ExcelResource + 'a, options: ExcelOptions) -> Self {
		Self {
			default_language: options.default_language,
			resource: Rc::new(resource),

			list: None.into(),
		}
	}

	pub fn sheet_reader(&self, sheet_name: &str) -> Result<SheetReader> {
		let list = self.list()?;

		if !list.has_sheet(sheet_name) {
			return Err(Error::NotFound(format!("Sheet \"{}\"", sheet_name)));
		}

		// todo: possibly should cache the raw sheets
		let sheet = SheetReader::with_options(
			sheet_name,
			self.resource.clone(),
			SheetOptions {
				default_language: self.default_language,
			},
		);

		Ok(sheet)
	}

	fn list(&self) -> Result<Rc<List>> {
		match &mut *self.list.borrow_mut() {
			Some(list) => Ok(list.clone()),
			option @ None => {
				let bytes = self.resource.list()?;
				let list = List::from_bytes(bytes)?;
				Ok(option.insert(Rc::new(list)).clone())
			}
		}
	}
}
