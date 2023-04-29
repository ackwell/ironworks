use crate::{
	error::{Error, ErrorValue, Result},
	sestring::{context::Context, expression::Expression},
};

use super::payload::Payload;

pub struct NewLine;

impl Payload for NewLine {
	fn resolve(&self, arguments: &[Expression], _context: &mut Context) -> Result<String> {
		if !arguments.is_empty() {
			return Err(Error::Invalid(
				// Should i have a sestring error value? maybe once i add a feature i guess?
				ErrorValue::Other("SeString".into()),
				format!("NewLine expected 0 arguments, got {}", arguments.len()),
			));
		}

		Ok("\n".into())
	}
}
