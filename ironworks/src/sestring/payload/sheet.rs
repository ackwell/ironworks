use crate::{
	error::Result,
	sestring::{
		context::Context,
		expression::Expression,
		value::{ArgumentExt, TryFromValue, Value},
	},
};

use super::payload::Payload;

pub struct Sheet;
impl Payload for Sheet {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		// TODO: column is optional i think, but need to check what the default col id is.
		let (_sheet, row, column, _parameter) =
			arguments.resolve::<(String, Value, Value, Option<u32>)>(context)?;

		// The row can be set to a string expression for further sheet payload lookups
		// (ref. addon@ja:111/0), which leaves us in a bit of a predicament, as
		// returning UNKNOWN as a string would bubble up the unknown state correctly,
		// but result in top-level sheet payloads (common in ja data) to render dodgy
		// numbers in place of strings (or nothing). Instead, we're special casing,
		// and assuming that emptystring is UNKNOWN.
		let row = match row {
			Value::U32(number) => number,
			Value::String(string) if string.is_empty() => Value::UNKNOWN,
			value @ Value::String(_) => u32::try_from_value(Some(value))?,
		};

		let column = match column {
			Value::U32(number) => number,
			Value::String(string) => match context.constant(&string) {
				Some(value) => value,
				None => u32::try_from_value(Some(Value::String(string)))?,
			},
		};

		// If the row or column are unknown, there's nothing we can realistically resolve, drop early.
		if row == Value::UNKNOWN || column == Value::UNKNOWN {
			return Ok("".into());
		}

		// TODO: resolve excel from context - this will require careful feature gates
		Ok("".into())
	}
}

pub struct AutoTranslate;
impl Payload for AutoTranslate {
	fn resolve(&self, _arguments: &[Expression], _context: &mut Context) -> Result<String> {
		// TODO: do lookup in excel
		Ok("".into())
	}
}
