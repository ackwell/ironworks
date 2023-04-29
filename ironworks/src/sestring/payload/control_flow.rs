use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct IfSelf;

impl Payload for IfSelf {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (_player_id, branch_true, _branch_false) =
			arguments.resolve::<(u32, String, String)>(context)?;

		// TODO: this is just assuming that every player id is the player - i'll need to decide how to handle this conceptually - maybe a faux `IfSelf::PLAYER_ID`? but that'd mean assuming which param is the player ID, which isn't safe

		Ok(branch_true)
	}
}
