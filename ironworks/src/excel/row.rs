use std::{
	io::Cursor,
	sync::{Arc, Mutex},
};

use binrw::{BinReaderExt, BinResult};

use crate::{
	error::{Error, ErrorValue, Result},
	excel::field::Field,
	file::exh,
	sestring::SeString,
};

/// Specifier for targeting a single column within a sheet.
#[derive(Debug)]
pub enum ColumnSpecifier<'a> {
	/// Specifies the column at the Nth index within the sheet's column array.
	Index(usize),
	/// Specifies the column with the provided definition.
	Definition(&'a exh::ColumnDefinition),
}

impl From<usize> for ColumnSpecifier<'_> {
	fn from(index: usize) -> Self {
		Self::Index(index)
	}
}

impl<'a> From<&'a exh::ColumnDefinition> for ColumnSpecifier<'a> {
	fn from(definition: &'a exh::ColumnDefinition) -> Self {
		Self::Definition(definition)
	}
}

/// A (sub)row within an Excel sheet.
#[derive(Debug)]
pub struct Row {
	row_id: u32,
	subrow_id: u16,

	header: Arc<exh::ExcelHeader>,
	data: Mutex<Cursor<Vec<u8>>>,
}

impl Row {
	pub(super) fn new(
		row_id: u32,
		subrow_id: u16,
		header: Arc<exh::ExcelHeader>,
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
	pub fn row_id(&self) -> u32 {
		self.row_id
	}

	/// Subrow ID of this row.
	pub fn subrow_id(&self) -> u16 {
		self.subrow_id
	}

	/// Read the field at the specified column from this row.
	pub fn field<'a>(&self, specifier: impl Into<ColumnSpecifier<'a>>) -> Result<Field> {
		let column = match specifier.into() {
			ColumnSpecifier::Definition(definition) => definition,
			ColumnSpecifier::Index(index) => {
				self.header.columns().get(index).ok_or_else(|| {
					// TODO: should this have its own value type?
					Error::NotFound(ErrorValue::Other(format!("Column {index}")))
				})?
			}
		};

		Ok(self.read_field(column)?)
	}

	fn read_field(&self, column: &exh::ColumnDefinition) -> BinResult<Field> {
		use exh::ColumnKind as K;
		use Field as F;

		let mut cursor = self.data.lock().expect("Data mutex poisoned.");

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
