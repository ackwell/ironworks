use crate::sestring2::error::Result;

use super::{argument::Arguments, format::State, value::Value};

// TODO: Implement these.

pub fn sheet<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (_sheet, _row, _column, _parameter) =
		arguments.exhaustive::<(String, u32, Option<u32>, Option<u32>)>(state)?;
	Ok(())
}

pub fn sound<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (_is_jingle, _id) = arguments.exhaustive::<(bool, u32)>(state)?;
	Ok(())
}

pub fn level_pos<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let _id = arguments.exhaustive::<u32>(state)?;
	Ok(())
}

pub fn ja_noun<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	noun("ja", arguments, state)
}

pub fn en_noun<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	noun("en", arguments, state)
}

pub fn de_noun<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	noun("de", arguments, state)
}

pub fn fr_noun<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	noun("fr", arguments, state)
}

pub fn ch_noun<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	noun("ch", arguments, state)
}

// TODO: this should use an excel language
fn noun<'a>(_lang: &str, arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (_sheet, _person, _row, _amount, _case, _unknown) =
		arguments.exhaustive::<(String, u32, u32, u32, Value, Option<Value>)>(state)?;
	Ok(())
}
