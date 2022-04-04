use crate::{
	error::{Error, ErrorValue, Result},
	utility::{OptionCache, OptionCacheExt},
};

use super::{list::List, resource::Resource, sheet::Sheet};

/// An excel database.
#[derive(Debug)]
pub struct Excel<R> {
	resource: R,

	list: OptionCache<List>,
}

impl<R: Resource> Excel<R> {
	/// Build a representation of an Excel database.
	pub fn new(resource: R) -> Self {
		Self {
			resource,
			list: Default::default(),
		}
	}

	/// Fetch a sheet from the database.
	pub fn sheet(&self, sheet: &str) -> Result<Sheet<R>> {
		let list = self
			.list
			.try_get_or_insert(|| List::read(self.resource.list()?))?;
		if !list.has(sheet) {
			return Err(Error::NotFound(ErrorValue::Sheet(sheet.into())));
		}

		Ok(Sheet::new(sheet.into(), &self.resource))
	}
}
