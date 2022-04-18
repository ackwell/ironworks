use crate::sestring::SeString;

/// A single field from an Excel database.
#[allow(missing_docs)]
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

// TODO: Think about this. It maps with how serde json does it so it can't be all bad.
// TODO: should i also expose as_ for these (refs)?
// TODO: this is repetetive and dumb, look into a macro or crate for this shit.
impl Field {
	/// Attempt to consume this field and return a `SeString`. Returns `Err(self)` if this field is not a `SeString`.
	pub fn into_string(self) -> Result<SeString, Self> {
		match self {
			Self::String(value) => Ok(value),
			_ => Err(self),
		}
	}

	/// Attempt to consume this field and return a `bool`. Returns `Err(self)` if this field is not a `bool`.
	pub fn into_bool(self) -> Result<bool, Self> {
		match self {
			Self::Bool(value) => Ok(value),
			_ => Err(self),
		}
	}

	// TODO: Rest

	/// Attempt to consume this field and return a `u32`. Returns `Err(self)` if this field is not a `u32`.
	pub fn into_u32(self) -> Result<u32, Self> {
		match self {
			Self::U32(value) => Ok(value),
			_ => Err(self),
		}
	}

	// TODO: Rest
}
