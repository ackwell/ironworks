use crate::{
	error::Result,
	excel::Language,
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
			arguments.resolve::<(String, u32, Option<Value>, Option<u32>)>(context)?;

		let column = match column {
			None => 0,
			Some(Value::U32(number)) => number,
			Some(Value::String(string)) => match context.constant(&string) {
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

pub struct Noun(pub Language);
impl Payload for Noun {
	fn resolve(&self, _arguments: &[Expression], _context: &mut Context) -> Result<String> {
		// TODO: do lookup in excel. this has a _whole_ lot of messy stuff w/r/t attributives and so on.
		Ok("".into())
	}
}
