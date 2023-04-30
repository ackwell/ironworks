use crate::{error::Result, Error, ErrorValue};

use super::{context::Context, expression::Expression};

#[derive(Debug)]
pub enum Value {
	U32(u32),
	String(String),
}

impl Value {
	/// Representation of a u32-kind unknown value, used as the default value for
	/// unspecified parameters. It is treated as an always-successful condition.
	pub const UNKNOWN: u32 = u32::MAX;
}

// Baking my own TryFrom so I don't need a newtype for option.
pub trait TryFromValue: Sized {
	fn try_from_value(value: Option<Value>) -> Result<Self>;
}

impl TryFromValue for Value {
	fn try_from_value(value: Option<Value>) -> Result<Self> {
		match value {
			Some(value) => Ok(value),
			None => not_enough_arguments(),
		}
	}
}

impl TryFromValue for u32 {
	fn try_from_value(value: Option<Value>) -> Result<Self> {
		match value {
			Some(Value::U32(number)) => Ok(number),
			// This... doesn't really make sense, but there's real game data (addon@jp:29/0)
			// that has a string expression in a numeric position. It's almost certainly
			// a bug in the game, but it also doesn't crash (presumably) on that string,
			// so I guess we're handling that case now. This implementation is a bit
			// of a guess, but a nonsensical case gets a nonsensical impl so whatever.
			Some(Value::String(string)) => string.trim().parse::<u32>().map_err(|error| {
				Error::Invalid(
					ErrorValue::SeString,
					format!("could not coerce string to u32: {error}"),
				)
			}),
			None => not_enough_arguments(),
		}
	}
}

impl TryFromValue for String {
	fn try_from_value(value: Option<Value>) -> Result<Self> {
		match value {
			Some(Value::String(string)) => Ok(string),
			Some(Value::U32(number)) => Ok(number.to_string()),
			None => not_enough_arguments(),
		}
	}
}

impl<T> TryFromValue for Option<T>
where
	T: TryFromValue,
{
	fn try_from_value(value: Option<Value>) -> Result<Self> {
		match value {
			None => Ok(None),
			some => T::try_from_value(some).map(Some),
		}
	}
}

fn not_enough_arguments<T>() -> Result<T> {
	Err(Error::Invalid(
		ErrorValue::SeString,
		"insufficient arguments".into(),
	))
}

pub trait ArgumentExt {
	fn resolve<T>(&self, context: &mut Context) -> Result<T>
	where
		T: FromArguments;
}

impl ArgumentExt for &[Expression] {
	fn resolve<T>(&self, context: &mut Context) -> Result<T>
	where
		T: FromArguments,
	{
		T::from_arguments(self, context)
	}
}

pub trait FromArguments: Sized {
	fn from_arguments(arguments: &[Expression], context: &mut Context) -> Result<Self>;
}

impl FromArguments for () {
	fn from_arguments(arguments: &[Expression], _context: &mut Context) -> Result<Self> {
		check_exhausted(&mut arguments.iter())
	}
}

impl<T> FromArguments for T
where
	T: TryFromValue,
{
	fn from_arguments(arguments: &[Expression], context: &mut Context) -> Result<Self> {
		let iter = &mut arguments.iter();
		let value = resolve_argument(iter, context)?;
		check_exhausted(iter)?;
		Ok(value)
	}
}

macro_rules! tuple_impl {
	($arg:ident $(, $args:ident)*) => {
		#[allow(non_camel_case_types)]
		impl<$arg: TryFromValue, $($args: TryFromValue),*> FromArguments for ($arg, $($args),*) {
			fn from_arguments(arguments: &[Expression], context: &mut Context) -> Result<Self> {
				let iter = &mut arguments.iter();
				let result = (
					resolve_argument::<$arg>(iter, context)?,
					$(resolve_argument::<$args>(iter, context)?),*
				);
				check_exhausted(iter)?;
				Ok(result)
			}
		}

		tuple_impl!($($args),*);
	};

	() => {};
}

tuple_impl!(arg1, arg2, arg3, arg4);

fn resolve_argument<'a, T>(
	iter: &mut impl Iterator<Item = &'a Expression>,
	context: &mut Context,
) -> Result<T>
where
	T: TryFromValue,
{
	let expression = iter
		.next()
		.map(|expression| expression.resolve::<Value>(context))
		.transpose()?;
	T::try_from_value(expression)
}

fn check_exhausted<'a>(iter: &mut impl Iterator<Item = &'a Expression>) -> Result<()> {
	match iter.next() {
		None => Ok(()),
		Some(_) => Err(Error::Invalid(
			ErrorValue::SeString,
			"too many arguments".into(),
		)),
	}
}
