use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct Sheet;
impl Payload for Sheet {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (sheet, row, column, parameter) =
			arguments.resolve::<(String, u32, Option<u32>, Option<u32>)>(context)?;
		todo!("{sheet} {row} {column:?} {parameter:?}")
	}
}
