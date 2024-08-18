use crate::sestring2::error::{Error, Result};

use super::{
	argument::Arguments,
	format::{format_expression, State},
};

// TODO: These checks test against runtime data. How do?

pub fn pc_name<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let _object_id = arguments.exhaustive::<u32>(state)?;

	// TODO: old impl fetched from a discrete player name list. how should new impl handle it?

	Ok(())
}

pub fn if_self<'a>(mut arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let _object_id = arguments.evaluate::<u32>(state)?;

	// TODO: check
	format_branch(true, arguments, state)
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
