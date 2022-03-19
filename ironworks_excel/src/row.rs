use std::{io::Cursor, rc::Rc};

use binrw::{BinRead, BinReaderExt, NullString};

use crate::{
	error::Result,
	header::{ExcelColumnDefinition, ExcelColumnKind, ExcelHeader},
	Error,
};

// TODO put this somewhere sensible
#[derive(BinRead, Debug)]
#[br(big)]
pub struct ExcelRowHeader {
	pub data_size: u32,
	row_count: u16,
}

#[derive(Debug)]
pub enum ExcelField {
	String(SeString),

	Bool(bool),

	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),

	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),

	F32(f32),
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

	pub fn field(&self, column_index: u32) -> Result<ExcelField> {
		// get column definition
		let column = self
			.header
			.columns
			.get(column_index as usize)
			.ok_or_else(|| Error::NotFound(format!("Column {}", column_index)))?;

		// TODO: do we want to store the cursor in the main struct? might help with auto advancing rows... but at the same time, columns are not in byte order nessicarily
		let mut cursor = Cursor::new(&self.data);
		cursor.set_position(column.offset.into());

		self.read_field(column, &mut cursor).map_err(|error| {
			Error::InvalidResource(format!(
				"Failed to read {:?} at position {}: {}",
				column.kind, column.offset, error
			))
		})
	}

	fn read_field(
		&self,
		column: &ExcelColumnDefinition,
		mut cursor: &mut Cursor<&Vec<u8>>,
	) -> Result<ExcelField, binrw::Error> {
		match column.kind {
			ExcelColumnKind::String => {
				let string_offset = cursor.read_be::<u32>()?;
				cursor.set_position(string_offset as u64 + self.header.row_size as u64);
				let string = SeString::read(&mut cursor)?;
				Ok(ExcelField::String(string))
			}

			ExcelColumnKind::Bool => Ok(ExcelField::Bool(cursor.read_be::<u8>()? != 0)),
			ExcelColumnKind::PackedBool0
			| ExcelColumnKind::PackedBool1
			| ExcelColumnKind::PackedBool2
			| ExcelColumnKind::PackedBool3
			| ExcelColumnKind::PackedBool4
			| ExcelColumnKind::PackedBool5
			| ExcelColumnKind::PackedBool6
			| ExcelColumnKind::PackedBool7 => {
				let mask = 1 << (column.kind as u16 - ExcelColumnKind::PackedBool0 as u16);
				let value = cursor.read_be::<u8>()?;
				Ok(ExcelField::Bool((value & mask) == mask))
			}

			ExcelColumnKind::Int8 => Ok(ExcelField::I8(cursor.read_be::<i8>()?)),
			ExcelColumnKind::Int16 => Ok(ExcelField::I16(cursor.read_be::<i16>()?)),
			ExcelColumnKind::Int32 => Ok(ExcelField::I32(cursor.read_be::<i32>()?)),
			ExcelColumnKind::Int64 => Ok(ExcelField::I64(cursor.read_be::<i64>()?)),

			ExcelColumnKind::UInt8 => Ok(ExcelField::U8(cursor.read_be::<u8>()?)),
			ExcelColumnKind::UInt16 => Ok(ExcelField::U16(cursor.read_be::<u16>()?)),
			ExcelColumnKind::UInt32 => Ok(ExcelField::U32(cursor.read_be::<u32>()?)),
			ExcelColumnKind::UInt64 => Ok(ExcelField::U64(cursor.read_be::<u64>()?)),

			ExcelColumnKind::Float32 => Ok(ExcelField::F32(cursor.read_be::<f32>()?)),
		}
	}
}

// TODO: this shouldn't be here
#[derive(BinRead, Debug)]
#[br(big)]
pub struct SeString {
	raw: NullString,
}
