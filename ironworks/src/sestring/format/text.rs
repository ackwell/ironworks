use crate::sestring::error::Result;

use super::{argument::Arguments, format::State};

pub fn caps<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let input = arguments.exhaustive::<String>(state)?;
	state.writer.write_str(&input.to_uppercase())?;
	Ok(())
}

pub fn head<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	// TODO: LogMessage@German 4148:0 suggests that formatting inside text
	// formatters is valid. I'm currently nooping it in the evaluator, but a more
	// hollistic approach would likely require a) tracking text formatting in
	// state, and b) using a state method wrapper around the writer to apply
	// current text formats before writing output strings.
	let input = arguments.exhaustive::<String>(state)?;
	state.writer.write_str(&head_str(&input))?;
	Ok(())
}

pub fn head_all<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let input = arguments.exhaustive::<String>(state)?;
	let output = input.split_inclusive(' ').map(head_str).collect::<String>();
	state.writer.write_str(&output)?;
	Ok(())
}

fn head_str<'a>(input: &str) -> String {
	let mut chars = input.chars();
	match chars.next() {
		Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
		None => input.to_string(),
	}
}

pub fn lower_head<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let input = arguments.exhaustive::<String>(state)?;

	let mut chars = input.chars();
	let output = match chars.next() {
		Some(char) => char.to_lowercase().collect::<String>() + chars.as_str(),
		None => input,
	};

	state.writer.write_str(&output)?;

	Ok(())
}

pub fn lower<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let input = arguments.exhaustive::<String>(state)?;
	state.writer.write_str(&input.to_lowercase())?;
	Ok(())
}

pub fn split<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (string, pattern, index) = arguments.exhaustive::<(String, String, u32)>(state)?;

	let output = string
		.split(&pattern)
		.nth(index.try_into().unwrap())
		.unwrap_or("");

	state.writer.write_str(&output)?;

	Ok(())
}

pub fn ruby<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (string, pronounciation) = arguments.exhaustive::<(String, String)>(state)?;

	// TODO: this should realistically be exposed as a discrete call to the writer
	// so they can do the proper layout, but that can be a later problem.
	state
		.writer
		.write_str(&format!("{string} ({pronounciation})"))?;

	Ok(())
}

#[cfg(test)]
mod test {
	use crate::sestring::{
		expression::Expression,
		format::test::{resolve, str},
	};

	#[test]
	fn caps() {
		assert_eq!(resolve(super::caps, [Ok(str(b"eeby jeeby"))]), "EEBY JEEBY");
	}

	#[test]
	fn head() {
		assert_eq!(resolve(super::head, [Ok(str(b"eeby jeeby"))]), "Eeby jeeby");
	}

	#[test]
	fn head_all() {
		assert_eq!(
			resolve(super::head_all, [Ok(str(b"eeby jeeby"))]),
			"Eeby Jeeby"
		);
	}

	#[test]
	fn lower_head() {
		assert_eq!(
			resolve(super::lower_head, [Ok(str(b"EEBY JEEBY"))]),
			"eEBY JEEBY"
		);
	}

	#[test]
	fn lower() {
		assert_eq!(
			resolve(super::lower, [Ok(str(b"EEBY JEEBY"))]),
			"eeby jeeby"
		);
	}

	#[test]
	fn split() {
		assert_eq!(
			resolve(
				super::split,
				[
					Ok(str(b"zero one two")),
					Ok(str(b" ")),
					Ok(Expression::U32(1))
				]
			),
			"one"
		);
	}
}
