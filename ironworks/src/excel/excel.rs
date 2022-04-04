use std::cell::RefCell;

use crate::error::{Error, ErrorValue, Result};

use super::{list::List, resource::Resource, sheet::Sheet};

/// An excel database.
#[derive(Debug)]
pub struct Excel<R> {
	resource: R,

	list: RefCell<Option<List>>,
}

impl<R: Resource> Excel<R> {
	/// Build a representation of an Excel database.
	pub fn new(resource: R) -> Self {
		Self {
			resource,

			list: None.into(),
		}
	}

	/// Fetch a sheet from the database.
	pub fn sheet(&self, sheet: &str) -> Result<Sheet<R>> {
		if !self.list_has(sheet)? {
			return Err(Error::NotFound(ErrorValue::Sheet(sheet.into())));
		}

		Ok(Sheet::new(sheet.into(), &self.resource))
	}

	fn list_has(&self, sheet: &str) -> Result<bool> {
		let mut list_cell = self.list.borrow_mut();
		let list = match &mut *list_cell {
			Some(list) => list,
			option @ None => {
				let list = List::read(self.resource.list()?)?;
				option.insert(list)
			}
		};

		Ok(list.has(sheet))
	}
}
