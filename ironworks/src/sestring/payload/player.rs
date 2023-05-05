use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct PlayerName;
impl Payload for PlayerName {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let player_id = arguments.resolve::<u32>(context)?;
		Ok(context.player_name(player_id))
	}
}
