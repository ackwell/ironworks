use std::{io::Cursor, rc::Rc};

use binrw::{BinRead, BinReaderExt, NullString};

use crate::{
	error::{Error, Result},
	excel::ExcelResource,
	header::ExcelHeader,
	page::ExcelPage,
};

// TODO should this be ExcelRawSheet?
#[derive(Debug)]
pub struct RawExcelSheet<'a> {
	sheet_name: String,

	resource: Rc<dyn ExcelResource + 'a>,
}

impl<'a> RawExcelSheet<'a> {
	pub fn new(sheet_name: &str, resource: Rc<dyn ExcelResource + 'a>) -> Self {
		Self {
			sheet_name: sheet_name.into(),
			resource,
		}
	}

	// todo iterable rows?
	// todo: lang?

	pub fn get_row(&self, row_id: u32) -> Result<RowReader> {
		self.get_subrow(row_id, 0)
	}

	pub fn get_subrow(&self, row_id: u32, subrow_id: u32) -> Result<RowReader> {
		let header = self.get_header()?;

		// Find the page definition for the requested row, if any.
		let page_definition = header
			.pages
			.iter()
			.find(|page| page.start_id <= row_id && page.start_id + page.row_count > row_id)
			.ok_or_else(|| Error::NotFound(format!("Row ID \"{}\"", row_id)))?;

		let page = self.get_page(page_definition.start_id)?;

		// Find the row definition for the requested row. A failure here implies
		// corrupt resources.
		let row_definition = page
			.header
			.rows
			.iter()
			.find(|row| row.row_id == row_id)
			.expect("Requested row ID is not defined by the provided page.");

		// Read the row's header.
		// TODO: handle subrows + validation
		let mut cursor = Cursor::new(&page.data);
		cursor.set_position(row_definition.offset.into());
		let row_header = ExcelRowHeader::read(&mut cursor).unwrap();

		// Slice the page data for just the requested row.
		let offset = cursor.position() as usize;
		// TODO: Check data_length behavior on a subrow sheet.
		let length = header.data_offset as usize + row_header.data_size as usize;
		let data = &page.data[offset..offset + length];

		let row_reader = RowReader::new(data);

		Ok(row_reader)
	}

	fn get_header(&self) -> Result<ExcelHeader> {
		// todo: cache
		let bytes = self.resource.header(&self.sheet_name)?;
		let header = ExcelHeader::from_bytes(bytes)?;
		Ok(header)
	}

	fn get_page(&self, start_id: u32) -> Result<ExcelPage> {
		// TODO: this _needs_ to handle language
		// TODO: cache
		let bytes = self.resource.page(&self.sheet_name, start_id)?;
		let page = ExcelPage::from_bytes(bytes)?;
		Ok(page)
	}
}

// TODO put this somewhere sensible
#[derive(BinRead, Debug)]
#[br(big)]
struct ExcelRowHeader {
	data_size: u32,
	row_count: u16,
}

// TODO this is basically a raw row - standardise naming with the raw sheet. do we have a sheetreader and rowreader, or rawsheet and rawrow, or...
#[derive(Debug)]
pub struct RowReader {
	data: Vec<u8>,
}

impl RowReader {
	fn new(data: &[u8]) -> Self {
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
