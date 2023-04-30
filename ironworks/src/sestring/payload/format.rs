use crate::{
	error::Result,
	sestring::{
		context::Context,
		expression::Expression,
		value::{ArgumentExt, Value},
	},
};

use super::payload::Payload;

pub struct Identity;
impl Payload for Identity {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		arguments.resolve::<String>(context)
	}
}

pub struct Thousands;
impl Payload for Thousands {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (value, separator) = arguments.resolve::<(u32, String)>(context)?;

		// Unknown value shortcuts to 0 so we don't blast intmax all over the place.
		if value == Value::UNKNOWN {
			return Ok("0".into());
		}

		if value < 1000 {
			return Ok(value.to_string());
		}

		let left = (value as f32 / 1000.0).floor();
		let right = value % 1000;
		Ok(format!("{left}{separator}{right:03}"))
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use binrw::BinRead;

	use crate::sestring::SeString;

	use super::*;

	// TODO: this is disgusting
	fn str(bytes: &[u8]) -> Expression {
		Expression::String(SeString::read_le(&mut Cursor::new(bytes)).unwrap())
	}

	#[test]
	fn thousands_unknown() {
		assert_eq!(
			Thousands
				.resolve(
					&[Expression::U32(Value::UNKNOWN), str(b",")],
					&mut Context::default()
				)
				.unwrap(),
			"0"
		)
	}

	#[test]
	fn thousands_small() {
		assert_eq!(
			Thousands
				.resolve(&[Expression::U32(420), str(b",")], &mut Context::default())
				.unwrap(),
			"420"
		)
	}

	#[test]
	fn thousands_large() {
		assert_eq!(
			Thousands
				.resolve(
					&[Expression::U32(42069), str(b",")],
					&mut Context::default()
				)
				.unwrap(),
			"42,069"
		)
	}
}
