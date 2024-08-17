use crate::sestring2::error::Result;

use super::{argument::Arguments, context::Context, resolve::Resolve};

pub fn head<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let input = args.evaluate::<String>(resolver, context)?;
	Ok(head_str(&input))
}

pub fn head_all<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let input = args.evaluate::<String>(resolver, context)?;
	let output = input.split_inclusive(' ').map(head_str).collect::<String>();
	Ok(output)
}

fn head_str<'a>(input: &str) -> String {
	let mut chars = input.chars();
	match chars.next() {
		Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
		None => input.to_string(),
	}
}

pub fn lower_head<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let input = args.evaluate::<String>(resolver, context)?;
	let mut chars = input.chars();
	let output = match chars.next() {
		Some(char) => char.to_lowercase().collect::<String>() + chars.as_str(),
		None => input,
	};
	Ok(output)
}

pub fn lower<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let input = args.evaluate::<String>(resolver, context)?;
	Ok(input.to_lowercase())
}

pub fn split<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let (string, pattern, index) = args.evaluate::<(String, String, u32)>(resolver, context)?;
	let output = string
		.split(&pattern)
		.nth(index.try_into().unwrap())
		.unwrap_or("");
	Ok(output.into())
}

// todo: pronounciation/ruby

#[cfg(test)]
mod test {
	use crate::sestring2::{
		resolve::shared::test::{resolve, str},
		Expression,
	};

	use super::*;

	#[test]
	fn head_test() {
		assert_eq!(resolve(head, [Ok(str(b"eeby jeeby"))]), "Eeby jeeby");
	}

	#[test]
	fn head_all_test() {
		assert_eq!(resolve(head_all, [Ok(str(b"eeby jeeby"))]), "Eeby Jeeby");
	}

	#[test]
	fn lower_head_test() {
		assert_eq!(resolve(lower_head, [Ok(str(b"EEBY JEEBY"))]), "eEBY JEEBY");
	}

	#[test]
	fn lower_test() {
		assert_eq!(resolve(lower, [Ok(str(b"EEBY JEEBY"))]), "eeby jeeby");
	}

	#[test]
	fn split_test() {
		assert_eq!(
			resolve(
				split,
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
