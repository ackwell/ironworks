use std::{cell::RefCell, io::Cursor, rc::Rc};

use binrw::{binread, BinReaderExt, BinResult, NullString};

use crate::error::{Error, ErrorValue, Result};

use super::header::{ColumnKind, Header};

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

#[derive(Debug)]
pub enum Field {
	// TODO: SeString, somehow or another
	String(NullString),

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

/// A (sub)row within an Excel sheet.
#[derive(Debug)]
pub struct Row {
	row_id: u32,
	subrow_id: u16,

	header: Rc<Header>,
	data: RefCell<Cursor<Vec<u8>>>,
}

impl Row {
	pub(super) fn new(row_id: u32, subrow_id: u16, header: Rc<Header>, data: Vec<u8>) -> Self {
		Self {
			row_id,
			subrow_id,
			header,
			data: Cursor::new(data).into(),
		}
	}

	/// Row ID of this row.
	pub fn row_id(&self) -> &u32 {
		&self.row_id
	}

	/// Subrow ID of this row.
	pub fn subrow_id(&self) -> &u16 {
		&self.subrow_id
	}

	/// Read the field at the specified column from this row.
	pub fn field(&self, column_index: usize) -> Result<Field> {
		let column = self.header.columns.get(column_index).ok_or_else(|| {
			// TODO: should this have its own value type?
			Error::NotFound(ErrorValue::Other(format!("Column {column_index}")))
		})?;

		self.data.borrow_mut().set_position(column.offset.into());

		self.read_field(column.kind)
			.map_err(|error| Error::Resource(error.into()))
	}

	fn read_field(&self, kind: ColumnKind) -> BinResult<Field> {
		use ColumnKind as K;
		use Field as F;

		let mut cursor = self.data.borrow_mut();

		let field = match kind {
			K::String => {
				let string_offset = cursor.read_be::<u32>()?;
				cursor.set_position(u64::from(string_offset) + u64::from(self.header.row_size));
				F::String(cursor.read_be::<NullString>()?)
			}

			K::Bool => F::Bool(cursor.read_be::<u8>()? != 0),
			K::PackedBool0
			| K::PackedBool1
			| K::PackedBool2
			| K::PackedBool3
			| K::PackedBool4
			| K::PackedBool5
			| K::PackedBool6
			| K::PackedBool7 => {
				let mask = 1 << (u16::from(kind) - u16::from(K::PackedBool0));
				let value = cursor.read_be::<u8>()?;
				F::Bool((value & mask) == mask)
			}

			K::Int8 => F::I8(cursor.read_be::<i8>()?),
			K::Int16 => F::I16(cursor.read_be::<i16>()?),
			K::Int32 => F::I32(cursor.read_be::<i32>()?),
			K::Int64 => F::I64(cursor.read_be::<i64>()?),

			K::UInt8 => F::U8(cursor.read_be::<u8>()?),
			K::UInt16 => F::U16(cursor.read_be::<u16>()?),
			K::UInt32 => F::U32(cursor.read_be::<u32>()?),
			K::UInt64 => F::U64(cursor.read_be::<u64>()?),

			K::Float32 => F::F32(cursor.read_be::<f32>()?),
		};

		Ok(field)
	}
}
