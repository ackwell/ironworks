use binrw::BinRead;

use crate::{error::Result, excel::Resource};

use super::header::Header;

// TODO: consider lifetime vs Rc. Will depend if we want to allow sheets to live
// past the lifetime of the parent Excel instance.
/// A sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet<'r, R> {
	sheet: String,

	resource: &'r R,
}

impl<'r, R: Resource> Sheet<'r, R> {
	pub(crate) fn new(sheet: String, resource: &'r R) -> Self {
		Self { sheet, resource }
	}

	/// Fetch a row from this sheet by ID. In the case of a sheet with subrows,
	/// this will return subrow 0.
	pub fn row(&self, row_id: u32) -> Result<()> {
		self.subrow(row_id, 0)
	}

	// TODO: u16?
	/// Fetch a row from this sheet by its ID and subrow ID.
	pub fn subrow(&self, _row_id: u32, _subrow_id: u16) -> Result<()> {
		let mut header_reader = self.resource.header(&self.sheet)?;
		let header = Header::read(&mut header_reader);

		println!("header: {header:#?}");

		Ok(())
	}
}
