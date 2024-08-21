use crate::sestring::error::{Error, Result};

use super::{
	argument::Arguments,
	format::{format_expression, State},
	value::Value,
};

/// Representation of a player character object as utilised by string formatting.
#[derive(Debug, Clone)]
pub struct Player {
	/// Name of the player character. Names in FFXIV are required to be two words
	/// seperated by a single space. Failing to follow this convention may lead to
	/// malformed output.
	pub name: String,
	/// Gender of the player character.
	pub gender: Gender,
}

/// Gender options as utilised by string formatting.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
	// This is exclusively representative of gender as it exists in sestring
	// formatting logic. I wish they supported more options, too.
	Male,
	Female,
}

pub fn pc_name<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	// This is letting unknown fall back to 0 - is that okay?
	let object_id = arguments.exhaustive::<u32>(state)?;
	let player = state.input.player(object_id);
	state.writer.write_str(&player.name)?;
	Ok(())
}

// Untested; No usages in excel as of 2024-08-20.
pub fn if_pc_gender<'a>(mut arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let object_id = arguments.evaluate::<u32>(state)?;
	let player = state.input.player(object_id);

	let condition = match player.gender {
		Gender::Male => true,
		Gender::Female => false,
	};

	format_branch(condition, arguments, state)
}

// Untested; No usages in excel as of 2024-08-20.
pub fn if_pc_name<'a>(mut arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (object_id, name) = arguments.evaluate::<(u32, String)>(state)?;
	let player = state.input.player(object_id);

	let condition = player.name == name;

	format_branch(condition, arguments, state)
}

pub fn if_self<'a>(mut arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let object_id = arguments.evaluate::<Value>(state)?;
	let player_id = state.input.local_player_id();

	let condition = match (object_id, player_id) {
		// If we don't know either side of the equation, assume that there's a match.
		(Value::Unknown, _) | (_, None) => true,
		// We know both sides, coerce to u32 and compare.
		(left, Some(right)) => u32::from(left) == right,
	};

	format_branch(condition, arguments, state)
}

// From control_flow
fn format_branch<'a>(
	condition: bool,
	mut arguments: impl Arguments<'a>,
	state: &mut State,
) -> Result<()> {
	let branch = arguments
		.nth(match condition {
			true => 0,
			false => 1,
		})
		.transpose()?
		.ok_or(Error::InsufficientArguments)?;

	format_expression(branch, state)
}
