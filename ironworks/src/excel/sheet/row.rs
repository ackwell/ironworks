use std::{io::Cursor, rc::Rc};

use binrw::binread;

use crate::error::{Error, ErrorValue, Result};

use super::header::Header;

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct RowHeader {
	pub data_size: u32,
	pub row_count: u16,
}

#[binread]
#[derive(Debug)]
#[br(big)]
pub struct SubrowHeader {
	pub id: u16,
}

impl SubrowHeader {
	pub const SIZE: u16 = 2;
}

/// A (sub)row within an Excel sheet.
#[derive(Debug)]
pub struct Row {
	// TODO: do we make these public or use fns
	row_id: u32,
	subrow_id: u16,

	header: Rc<Header>,
	data: Cursor<Vec<u8>>,
}

impl Row {
	pub(super) fn new(row_id: u32, subrow_id: u16, header: Rc<Header>, data: Vec<u8>) -> Self {
		Self {
			row_id,
			subrow_id,
			header,
			data: Cursor::new(data),
		}
	}

	pub fn field(&self, column_index: usize) -> Result<()> {
		let column = self.header.columns.get(column_index).ok_or_else(|| {
			// TODO: should this have its own value type?
			Error::NotFound(ErrorValue::Other(format!("Column {column_index}")))
		})?;

		println!("{column:#?}");

		Ok(())
	}
}
