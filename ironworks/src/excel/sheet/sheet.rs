use std::{cell::RefCell, rc::Rc};

use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::Resource,
};

use super::header::{Header, SheetKind};

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet<'r, R> {
	sheet: String,

	resource: &'r R,

	header: RefCell<Option<Rc<Header>>>,
}

impl<'r, R: Resource> Sheet<'r, R> {
	pub(crate) fn new(sheet: String, resource: &'r R) -> Self {
		Self {
			sheet,

			resource,

			header: Default::default(),
		}
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row: u32) -> Result<()> {
		self.subrow(row, 0)
	}

	// TODO: u16?
	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, row: u32, subrow: u16) -> Result<()> {
		let header = self.header()?;

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if header.kind != SheetKind::Subrows && subrow > 0 {
			return Err(Error::NotFound(ErrorValue::Row {
				row,
				subrow,
				sheet: self.sheet.clone(),
			}));
		}

		println!("header: {header:#?}");

		Ok(())
	}

	fn header(&self) -> Result<Rc<Header>> {
		let mut cell = self.header.borrow_mut();
		let header = match &mut *cell {
			Some(header) => header,
			option @ None => {
				let mut reader = self.resource.header(&self.sheet)?;
				let header =
					Header::read(&mut reader).map_err(|error| Error::Resource(error.into()))?;
				option.insert(header.into())
			}
		};

		Ok(header.clone())
	}
}
