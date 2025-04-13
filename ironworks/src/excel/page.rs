use std::{io::Cursor, sync::Arc};

use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::Row,
	file::{exd, exh},
};

pub enum RowSpecifier<'a> {
	Id(u32),
	Definition(&'a exd::RowDefinition),
}

pub enum SubrowSpecifier {
	Id(u16),
	Index(usize),
}

#[derive(Debug)]
pub struct Page {
	header: Arc<exh::ExcelHeader>,
	data: exd::ExcelData,
}

impl Page {
	// for iterator testing
	pub fn TEMP_DATA(&self) -> &exd::ExcelData {
		&self.data
	}

	pub fn new(header: Arc<exh::ExcelHeader>, data: exd::ExcelData) -> Self {
		Self { header, data }
	}

	pub fn row<'a>(
		&self,
		row_specifier: RowSpecifier<'a>,
		subrow_specifier: SubrowSpecifier,
	) -> Result<Row> {
		// Resolve the specifier into a concrete definition.
		let (row_id, row_definition) = match row_specifier {
			RowSpecifier::Id(id) => (id, self.row_definition(id)?),
			RowSpecifier::Definition(definition) => (definition.id, definition),
		};

		// Read in the row's header, and use to determine bounds of row data.
		let mut cursor = Cursor::new(&self.data.data);
		cursor.set_position(row_definition.offset.into());
		// TODO: should this be a non-binrw method?
		let row_header = exd::RowHeader::read(&mut cursor)?;

		let row_pos = usize::try_from(cursor.position()).unwrap();
		let row_len = usize::try_from(row_header.size).unwrap();
		let row_data = &self.data.data[row_pos..row_pos + row_len];

		let (subrow_id, field_buffer, string_buffer) =
			self.row_buffers(row_data, row_header.count, subrow_specifier)?;

		// TODO: This means I'm cloning the entire row byte array each time, even if someone's asking for 2 fields. Perhaps consider using a "row reader" that operates on a temporary lifetime with the byte slice, and only to_vec the data in a concrete Row for raw reading?
		let row = Row::new(
			row_id,
			subrow_id,
			self.header.clone(),
			field_buffer.to_vec(),
			string_buffer.to_vec(),
		);

		Ok(row)
	}

	fn row_definition(&self, row_id: u32) -> Result<&exd::RowDefinition> {
		let Self { data, .. } = self;

		// Most pages are contiguous IDs - check if this assumption holds in this
		// case, and fast track if it does.
		let first_row_id = data.rows.get(0).map_or(0, |row| row.id);
		if let Some(index) = row_id.checked_sub(first_row_id) {
			let index_usize = usize::try_from(index).unwrap();
			if index_usize < data.rows.len() && data.rows[index_usize].id == row_id {
				return Ok(&data.rows[index_usize]);
			}
		}

		// Otherwise, fall back to a naive scan.
		data.rows
			.iter()
			.find(|row| row.id == row_id)
			.ok_or(Error::NotFound(ErrorValue::Row {
				row: row_id,
				subrow: 0,
				sheet: None,
			}))
	}

	fn row_buffers<'a>(
		&self,
		row_data: &'a [u8],
		subrow_count: u16,
		subrow_specifier: SubrowSpecifier,
	) -> Result<(u16, &'a [u8], &'a [u8])> {
		// For non-subrow sheets, there should be a single set of fields at offset 0,
		// followed by the string buffer.
		if self.header.kind != exh::SheetKind::Subrows {
			let (field_buffer, string_buffer) = row_data.split_at(self.header.row_size.into());
			return Ok((0, field_buffer, string_buffer));
		}

		let (subrow_id, subrow_index) = match subrow_specifier {
			SubrowSpecifier::Id(id) => (id, self.subrow_index(row_data, subrow_count, id)?),
			SubrowSpecifier::Index(index) => (self.subrow_id(row_data, index)?, index),
		};

		let fields_len = usize::from(self.header.row_size);
		let fields_pos = subrow_index * (exd::SubrowHeader::SIZE + fields_len);
		let strings_pos = usize::from(subrow_count) * (exd::SubrowHeader::SIZE + fields_len);

		Ok((
			subrow_id,
			&row_data[fields_pos..fields_pos + fields_len],
			&row_data[strings_pos..],
		))
	}

	fn subrow_index(&self, row_data: &[u8], subrow_count: u16, subrow_id: u16) -> Result<usize> {
		let mut cursor = Cursor::new(row_data);
		for index in 0..subrow_count {
			let subrow_header = exd::SubrowHeader::read(&mut cursor)?;
			if subrow_header.id == subrow_id {
				return Ok(index.into());
			}
			cursor.seek_relative(self.header.row_size.into())?;
		}

		// TODO: Man I _really_ need to rethink errors
		Err(Error::NotFound(ErrorValue::Row {
			// row: row_id,
			row: 0,
			subrow: subrow_id,
			sheet: None,
		}))
	}

	fn subrow_id(&self, row_data: &[u8], subrow_index: usize) -> Result<u16> {
		let mut cursor = Cursor::new(row_data);
		let subrow_pos =
			subrow_index * (exd::SubrowHeader::SIZE + usize::from(self.header.row_size));
		cursor.set_position(u64::try_from(subrow_pos).unwrap());
		let subrow_header = exd::SubrowHeader::read(&mut cursor)?;
		Ok(subrow_header.id)
	}
}
