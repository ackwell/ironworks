use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::Resource,
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
};

use super::{
	header::{Header, SheetKind},
	page::Page,
};

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet<'r, R> {
	sheet: String,

	resource: &'r R,

	header: OptionCache<Header>,
	pages: HashMapCache<(u32, u8), Page>,
}

impl<'r, R: Resource> Sheet<'r, R> {
	pub(crate) fn new(sheet: String, resource: &'r R) -> Self {
		Self {
			sheet,

			resource,

			header: Default::default(),
			pages: Default::default(),
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
		let header = self.header.try_get_or_insert(|| {
			let mut reader = self.resource.header(&self.sheet)?;
			Header::read(&mut reader).map_err(|error| Error::Resource(error.into()))
		})?;

		// Fail out early if a subrow >0 was requested on a non-subrow sheet.
		if header.kind != SheetKind::Subrows && subrow > 0 {
			return Err(Error::NotFound(ErrorValue::Row {
				row,
				subrow,
				sheet: self.sheet.clone(),
			}));
		}

		// Try to read in the page start ID for the requested (sub)row.
		let start_id = header
			.pages
			.iter()
			.find(|page| page.start_id <= row && page.start_id + page.row_count > row)
			.ok_or_else(|| {
				Error::NotFound(ErrorValue::Row {
					row,
					subrow,
					sheet: self.sheet.clone(),
				})
			})?
			.start_id;

		// TODO language
		let language = 1;
		let page = self.pages.try_get_or_insert((start_id, language), || {
			let mut reader = self.resource.page(&self.sheet, start_id, language)?;
			Page::read(&mut reader).map_err(|error| Error::Resource(error.into()))
		})?;

		println!("page: {page:?}");

		Ok(())
	}
}
