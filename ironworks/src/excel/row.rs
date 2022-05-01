use std::{cell::RefCell, io::Cursor, sync::Arc};

use binrw::{BinReaderExt, BinResult};

use crate::{
	error::{Error, ErrorValue, Result},
	excel::field::Field,
	file,
	sestring::SeString,
};

/// A (sub)row within an Excel sheet.
#[derive(Debug)]
pub struct Row {
	row_id: u32,
	subrow_id: u16,

	header: Arc<file::exh::ExcelHeader>,
	data: RefCell<Cursor<Vec<u8>>>,
}

impl Row {
	pub(super) fn new(
		row_id: u32,
		subrow_id: u16,
		header: Arc<file::exh::ExcelHeader>,
		data: Vec<u8>,
	) -> Self {
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

	// TODO: Perhaps expose this as column: impl IntoColumn so i.e. a coldef can be passed by a theoretical pre-byte-sort'd host
	/// Read the field at the specified column from this row.
	pub fn field(&self, column_index: usize) -> Result<Field> {
		let column = self.header.columns().get(column_index).ok_or_else(|| {
			// TODO: should this have its own value type?
			Error::NotFound(ErrorValue::Other(format!("Column {column_index}")))
		})?;

		self.read_field(column)
			.map_err(|error| Error::Resource(error.into()))
	}

	fn read_field(&self, column: &file::exh::ColumnDefinition) -> BinResult<Field> {
		use file::exh::ColumnKind as K;
		use Field as F;

		let mut cursor = self.data.borrow_mut();

		cursor.set_position(column.offset().into());

		let field = match column.kind() {
			K::String => {
				let string_offset = cursor.read_be::<u32>()?;
				cursor.set_position(u64::from(string_offset) + u64::from(self.header.row_size()));
				F::String(cursor.read_be::<SeString>()?)
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
				let mask = 1 << (u16::from(column.kind()) - u16::from(K::PackedBool0));
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
