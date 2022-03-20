use std::{io::Cursor, rc::Rc};

use binrw::{BinRead, BinReaderExt};

use crate::{
	error::Result,
	header::{ColumnDefinition, ColumnKind, Header},
	string::SeString,
	Error,
};

#[derive(BinRead, Debug)]
#[br(big)]
pub struct RowHeader {
	pub data_size: u32,
	pub row_count: u16,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct SubrowHeader {
	#[allow(dead_code)]
	subrow_id: u16,
}

impl SubrowHeader {
	pub const SIZE: usize = 2;
}

#[derive(Debug)]
pub enum Field {
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

#[derive(Debug)]
pub struct RowReader {
	pub row_id: u32,
	pub subrow_id: u16,

	header: Rc<Header>,
	data: Vec<u8>,
}

impl RowReader {
	pub fn new(row_id: u32, subrow_id: u16, header: Rc<Header>, data: &[u8]) -> Self {
		Self {
			row_id,
			subrow_id,
			header,
			data: data.to_vec(),
		}
	}

	pub fn field(&self, column_index: u32) -> Result<Field> {
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
		column: &ColumnDefinition,
		mut cursor: &mut Cursor<&Vec<u8>>,
	) -> Result<Field, binrw::Error> {
		match column.kind {
			ColumnKind::String => {
				let string_offset = cursor.read_be::<u32>()?;
				cursor.set_position(string_offset as u64 + self.header.row_size as u64);
				let string = SeString::read(&mut cursor)?;
				Ok(Field::String(string))
			}

			ColumnKind::Bool => Ok(Field::Bool(cursor.read_be::<u8>()? != 0)),
			ColumnKind::PackedBool0
			| ColumnKind::PackedBool1
			| ColumnKind::PackedBool2
			| ColumnKind::PackedBool3
			| ColumnKind::PackedBool4
			| ColumnKind::PackedBool5
			| ColumnKind::PackedBool6
			| ColumnKind::PackedBool7 => {
				let mask = 1 << (column.kind as u16 - ColumnKind::PackedBool0 as u16);
				let value = cursor.read_be::<u8>()?;
				Ok(Field::Bool((value & mask) == mask))
			}

			ColumnKind::Int8 => Ok(Field::I8(cursor.read_be::<i8>()?)),
			ColumnKind::Int16 => Ok(Field::I16(cursor.read_be::<i16>()?)),
			ColumnKind::Int32 => Ok(Field::I32(cursor.read_be::<i32>()?)),
			ColumnKind::Int64 => Ok(Field::I64(cursor.read_be::<i64>()?)),

			ColumnKind::UInt8 => Ok(Field::U8(cursor.read_be::<u8>()?)),
			ColumnKind::UInt16 => Ok(Field::U16(cursor.read_be::<u16>()?)),
			ColumnKind::UInt32 => Ok(Field::U32(cursor.read_be::<u32>()?)),
			ColumnKind::UInt64 => Ok(Field::U64(cursor.read_be::<u64>()?)),

			ColumnKind::Float32 => Ok(Field::F32(cursor.read_be::<f32>()?)),
		}
	}
}
