use std::io::Cursor;

use binrw::{BinRead, BinReaderExt, NullString};

use crate::{
	error::Result,
	header::{ExcelColumnDefinition, ExcelColumnKind},
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
#[derive(Debug)]
pub enum ExcelField {
	String(SeString),
}

// TODO this is basically a raw row - standardise naming with the raw sheet. do we have a sheetreader and rowreader, or rawsheet and rawrow, or...
#[derive(Debug)]
pub struct RowReader {
	// TODO: this should probably be an Rc
	columns: Vec<ExcelColumnDefinition>,
	data: Vec<u8>,
}

impl RowReader {
	pub fn new(columns: &[ExcelColumnDefinition], data: &[u8]) -> Self {
		Self {
			columns: columns.to_vec(),
			data: data.to_vec(),
		}
	}

	pub fn read_column(&self, column_index: u32) -> Result<ExcelField> {
		// get column definition
		let column = self
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
				// TODO: 28 is the row size... maybe rows should have a full rc ref of the header?
				cursor.set_position(string_offset as u64 + 28);
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
