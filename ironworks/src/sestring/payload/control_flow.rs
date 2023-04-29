use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct IfSelf;

impl Payload for IfSelf {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (player_id, branch_true, branch_false) =
			arguments.resolve::<(u32, String, String)>(context)?;

		// Both parameters and the player ID on the context default to Value::UNKNOWN,
		// so this effectively will always branch to true unless configuration is provided.
		let branch = match player_id == context.player_id() {
			true => branch_true,
			false => branch_false,
		};

		Ok(branch)
	}
}
