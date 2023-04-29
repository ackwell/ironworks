use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::Value},
};

pub trait Payload {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String>;
}

pub struct Fallback;

impl Payload for Fallback {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		// Given this is a fallback and therefore we do not know the semantics of
		// the arguments, err to collecting all valid string arguments and returning as-is.
		let string = arguments
			.iter()
			.filter_map(|argument| match argument.resolve(context) {
				Ok(Value::String(string)) => Some(Ok(string)),
				Ok(Value::U32(_)) => None,
				Err(error) => Some(Err(error)),
			})
			.collect::<Result<String>>()?;

		Ok(string)
	}
}
