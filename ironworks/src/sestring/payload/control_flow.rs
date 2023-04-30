use crate::{
	error::{Error, ErrorValue, Result},
	sestring::{
		context::Context,
		expression::Expression,
		value::{ArgumentExt, Value},
	},
};

use super::payload::Payload;

pub struct If;
impl Payload for If {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (condition, branch_true, branch_false) =
			arguments.resolve::<(u32, String, String)>(context)?;

		Ok(match condition > 0 {
			true => branch_true,
			false => branch_false,
		})
	}
}

pub struct IfSelf;
impl Payload for IfSelf {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (player_id, branch_true, branch_false) =
			arguments.resolve::<(u32, String, String)>(context)?;

		// Both parameters and the player ID on the context default to Value::UNKNOWN,
		// so this effectively will always branch to true unless configuration is provided.
		Ok(match player_id == context.player_id() {
			true => branch_true,
			false => branch_false,
		})
	}
}

pub struct Switch;
impl Payload for Switch {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let count = arguments.len();
		if count < 2 {
			return Err(Error::Invalid(
				ErrorValue::SeString,
				format!("at least two arguments required, received {count}"),
			));
		}

		let mut scrutinee = arguments[0].resolve(context)?;

		// If the value is unknown, pick the first case.
		if scrutinee == Value::UNKNOWN {
			scrutinee = 1;
		}

		let branch = arguments
			.get(usize::try_from(scrutinee).unwrap())
			.ok_or_else(|| {
				Error::Invalid(
					ErrorValue::SeString,
					format!("insufficient arguments, expected {scrutinee} found {count}"),
				)
			})?;

		branch.resolve(context)
	}
}
