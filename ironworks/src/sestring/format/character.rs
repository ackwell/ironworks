use crate::sestring::error::Result;

use super::{argument::Arguments, format::State, value::Value};

pub fn new_line<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	arguments.exhaustive::<()>(state)?;
	state.writer.write_str("\n")?;
	Ok(())
}

// AKA page separator?
pub fn key<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	// Absolutely no idea what this arg is used for. Seemingly receives a U32 in the 8.2m range _sometimes_.
	let _unknown = arguments.exhaustive::<Option<Value>>(state)?;
	state.writer.write_str("\n")?;
	Ok(())
}

pub fn soft_hyphen<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	arguments.exhaustive::<()>(state)?;
	state.writer.write_str("\u{00AD}")?;
	Ok(())
}

pub fn non_breaking_space<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	arguments.exhaustive::<()>(state)?;
	state.writer.write_str("\u{0020}")?;
	Ok(())
}

pub fn hyphen<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	arguments.exhaustive::<()>(state)?;
	state.writer.write_str("\u{2013}")?;
	Ok(())
}

// TODO: is there a better category for this?
pub fn icon<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let _id = arguments.exhaustive::<u32>(state)?;
	// TODO: how would this be output? something on the writer? we only have the ID here, and it'd need to be looked up in the fontdata, etc.
	Ok(())
}
