use crate::sestring2::error::Result;

use super::{
	argument::{Arguments, TryFromArgument},
	context::Context,
	resolve::Resolve,
	value::Value,
};

pub fn r#if<'a>(
	resolver: &mut impl Resolve,
	mut args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	// Ensure an explicit unknown is treated as truthy.
	let condition = match args.next_as::<Value>(resolver, context)? {
		Value::Unknown => true,
		other => u32::from(other) > 0,
	};

	let branch = args
		.nth(match condition {
			true => 0,
			false => 1,
		})
		.transpose()?;

	String::try_from_argument(branch, resolver, context)
}

pub fn switch<'a>(
	resolver: &mut impl Resolve,
	mut args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	// Explicitly control Unknown's value to point to the first branch.
	let scrutinee = match args.next_as::<Value>(resolver, context)? {
		Value::Unknown => 1,
		other => other.into(),
	};

	let branch = args
		.nth(usize::try_from(scrutinee - 1).unwrap())
		.transpose()?;

	String::try_from_argument(branch, resolver, context)
}

#[cfg(test)]
mod test {
	use crate::sestring2::{
		error::Error,
		expression::Expression,
		resolve::shared::test::{resolve, str},
	};

	use super::*;

	// can use FF03FFEC (string of string of stackcolor length) to trigger an error

	#[test]
	fn if_true() {
		assert_eq!(
			resolve(
				r#if,
				[
					Ok(Expression::U32(1)),
					Ok(str(b"true")),
					Err(Error::InvalidExpression)
				]
			),
			"true"
		);
	}

	#[test]
	fn if_false() {
		assert_eq!(
			resolve(
				r#if,
				[
					Ok(Expression::U32(0)),
					Ok(str(b"\xFF\xEC")),
					Ok(str(b"false"))
				]
			),
			"false"
		);
	}

	#[test]
	fn if_unknown() {
		assert_eq!(
			resolve(
				r#if,
				[
					Ok(Expression::U32(1)),
					Ok(str(b"unknown")),
					Err(Error::InvalidExpression)
				]
			),
			"unknown"
		);
	}

	#[test]
	fn switch_1() {
		assert_eq!(
			resolve(
				switch,
				[
					Ok(Expression::U32(1)),
					Ok(str(b"1")),
					Err(Error::InvalidExpression)
				]
			),
			"1"
		);
	}

	#[test]
	fn switch_3() {
		assert_eq!(
			resolve(
				switch,
				[
					Ok(Expression::U32(3)),
					Ok(Expression::U32(0)),
					Ok(str(b"\xFF\xEC")),
					Ok(str(b"3")),
					Err(Error::InvalidExpression)
				]
			),
			"3"
		);
	}

	#[test]
	fn switch_unknown() {
		// Checking behavior of unknown - this relies on StackColour being an unknown value to context.
		assert_eq!(
			resolve(
				switch,
				[
					Ok(Expression::StackColor),
					Ok(str(b"unknown")),
					Err(Error::InvalidExpression)
				]
			),
			"unknown"
		);
	}
}
