use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct NewLine;

impl Payload for NewLine {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		arguments.resolve::<()>(context)?;

		Ok("\n".into())
	}
}
