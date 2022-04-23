use crate::{
	error::{Error, ErrorValue, Result},
	utility::{OptionCache, OptionCacheExt},
};

use super::{
	excel_options::ExcelOptions, list::List, metadata::SheetMetadata, resource::Resource,
	sheet::Sheet,
};

/// An Excel database.
#[derive(Debug)]
pub struct Excel<R> {
	default_language: u8,

	resource: R,

	list: OptionCache<List>,
}

impl<R: Resource> Excel<R> {
	/// Build an Excel database.
	pub fn new(resource: R) -> Self {
		Self::with().build(resource)
	}

	/// Build an Excel database with additional options.
	pub fn with() -> ExcelOptions<R> {
		Default::default()
	}

	pub(super) fn with_options(resource: R, options: &ExcelOptions<R>) -> Self {
		Self {
			default_language: options.language.unwrap_or(0),

			resource,

			list: Default::default(),
		}
	}

	/// Get the version string of the database.
	pub fn version(&self) -> Result<String> {
		self.resource.version()
	}

	/// Fetch a sheet from the database.
	pub fn sheet<S: SheetMetadata>(&self, sheet_metadata: S) -> Result<Sheet<S, R>> {
		let sheet_name = sheet_metadata.name();

		let list = self
			.list
			.try_get_or_insert(|| List::read(self.resource.list()?))?;
		if !list.has(&sheet_name) {
			return Err(Error::NotFound(ErrorValue::Sheet(sheet_name)));
		}

		Ok(Sheet::new(
			sheet_metadata,
			self.default_language,
			&self.resource,
		))
	}
}
