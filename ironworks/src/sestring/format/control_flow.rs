use crate::sestring::error::{Error, Result};

use super::{
	argument::Arguments,
	format::{format_expression, State},
	value::Value,
};

pub fn r#if<'a>(mut arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	// Explicit unknown is treated as truthy.
	let condition = match arguments.evaluate::<Value>(state)? {
		Value::Unknown => true,
		other => other.into(),
	};

	format_branch(
		arguments,
		match condition {
			true => 0,
			false => 1,
		},
		state,
	)
}

pub fn switch<'a>(mut arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	// Control explicit Unknowns to point to the first branch.
	let scrutinee = match arguments.evaluate::<Value>(state)? {
		Value::Unknown => 1,
		// Some values may evaluate to 0 (i.e. missing data from sheets), just pick
		// the first branch as a fallback.
		other => u32::max(1, other.into()),
	};

	format_branch(arguments, usize::try_from(scrutinee - 1).unwrap(), state)
}

fn format_branch<'a>(
	mut arguments: impl Arguments<'a>,
	index: usize,
	state: &mut State,
) -> Result<()> {
	let branch = arguments
		.nth(index)
		.transpose()?
		.ok_or(Error::InsufficientArguments)?;

	format_expression(branch, state)
}

#[cfg(test)]
mod test {
	use crate::sestring::{
		expression::Expression,
		format::test::{resolve, str},
	};

	use super::*;

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
					Err(Error::InvalidExpression),
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
					Ok(Expression::StackColor),
					Ok(str(b"unknown")),
					Err(Error::InvalidExpression)
				]
			),
			"unknown"
		);
	}

	#[test]
	fn if_non_string_branch() {
		assert_eq!(
			resolve(
				r#if,
				[
					Ok(Expression::U32(1)),
					Ok(Expression::U32(220)),
					Err(Error::InvalidExpression)
				]
			),
			"220"
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
					Err(Error::InvalidExpression),
					Err(Error::InvalidExpression),
					Ok(str(b"3")),
					Err(Error::InvalidExpression)
				]
			),
			"3"
		);
	}

	#[test]
	fn switch_unknown() {
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
