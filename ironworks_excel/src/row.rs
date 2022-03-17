use std::{io::Cursor, rc::Rc};

use binrw::{BinRead, BinReaderExt, NullString};

use crate::{
	error::Result,
	header::{ExcelColumnKind, ExcelHeader},
	Error,
};

// TODO put this somewhere sensible
#[derive(BinRead, Debug)]
#[br(big)]
pub struct ExcelRowHeader {
	pub data_size: u32,
	row_count: u16,
}

// TODO: this name is pretty bad, think about it
//       mixed term between field and column atm
//       tempated to say "column" is only used to refer to a _full_ column, and field for fetching a single... field
#[derive(Debug)]
pub enum ExcelField {
	String(SeString),
}

// TODO this is basically a raw row - standardise naming with the raw sheet. do we have a sheetreader and rowreader, or rawsheet and rawrow, or...
#[derive(Debug)]
pub struct RowReader {
	header: Rc<ExcelHeader>,
	data: Vec<u8>,
}

impl RowReader {
	pub fn new(header: Rc<ExcelHeader>, data: &[u8]) -> Self {
		Self {
			header,
			data: data.to_vec(),
		}
	}

	pub fn read_column(&self, column_index: u32) -> Result<ExcelField> {
		// get column definition
		let column = self
			.header
			.columns
			.get(column_index as usize)
			.ok_or_else(|| Error::NotFound(format!("Column {}", column_index)))?;

		// TODO: do we want to store the cursor in the main struct? might help with auto advancing rows... but at the same time, columns are not in byte order nessicarily
		let mut cursor = Cursor::new(&self.data);
		cursor.set_position(column.offset.into());

		match column.kind {
			ExcelColumnKind::String => {
				// TODO: error handling
				let string_offset = cursor.read_be::<u32>().unwrap();
				cursor.set_position(string_offset as u64 + self.header.row_size as u64);
				let string = SeString::read(&mut cursor).unwrap();
				Ok(ExcelField::String(string))
			}
			_ => todo!("column kind {:?}", column.kind),
		}
	}
}

// TODO: this shouldn't be here
#[derive(BinRead, Debug)]
#[br(big)]
pub struct SeString {
	raw: NullString,
}
