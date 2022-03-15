use std::io::Cursor;

use binrw::{BinRead, BinReaderExt, NullString};

// TODO put this somewhere sensible
#[derive(BinRead, Debug)]
#[br(big)]
pub struct ExcelRowHeader {
	pub data_size: u32,
	row_count: u16,
}

// TODO this is basically a raw row - standardise naming with the raw sheet. do we have a sheetreader and rowreader, or rawsheet and rawrow, or...
#[derive(Debug)]
pub struct RowReader {
	data: Vec<u8>,
}

impl RowReader {
	pub fn new(data: &[u8]) -> Self {
		Self {
			data: data.to_owned(),
		}
	}

	pub fn temp_test(&self) -> SeString {
		// TODO: do we want to store the cursor in the main struct? might help with auto advancing rows... but at the same time, columns are not in byte order nessicarily
		let mut cursor = Cursor::new(&self.data);

		// todo: temp obv
		let column_offset = 0x10u64;
		cursor.set_position(column_offset);

		// read the string offset
		let string_offset = cursor.read_be::<u32>().unwrap();

		// read sestr from the offset pos
		// todo: how are we getting the 28 here?
		cursor.set_position(string_offset as u64 + 28);
		let string = SeString::read(&mut cursor).unwrap();

		return string;
	}
}

// TODO: this shouldn't be here
#[derive(BinRead, Debug)]
#[br(big)]
pub struct SeString {
	raw: NullString,
}
