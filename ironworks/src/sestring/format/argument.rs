use crate::sestring::{
	error::{Error, Result},
	expression::Expression,
};

use super::{expression::evaluate_expression, format::State, value::Value};

pub trait Arguments<'a>: Sized + Iterator<Item = Result<Expression<'a>>> {
	fn evaluate<T>(&mut self, state: &State) -> Result<T>
	where
		T: TryFromArguments<'a>,
	{
		T::try_from_arguments(self, state)
	}

	fn exhaustive<T>(mut self, state: &State) -> Result<T>
	where
		T: TryFromArguments<'a>,
	{
		let out = self.evaluate::<T>(state)?;
		match self.count() {
			0 => Ok(out),
			n => Err(Error::TooManyArguments(n)),
		}
	}
}

impl<'a, T> Arguments<'a> for T where T: Iterator<Item = Result<Expression<'a>>> {}

pub trait TryFromArguments<'a>: Sized {
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, state: &State) -> Result<Self>;
}

impl<'a> TryFromArguments<'a> for () {
	fn try_from_arguments(_arguments: &mut impl Arguments<'a>, _state: &State) -> Result<Self> {
		Ok(())
	}
}

impl<'a> TryFromArguments<'a> for Expression<'a> {
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, _state: &State) -> Result<Self> {
		arguments
			.next()
			.transpose()?
			.ok_or(Error::InsufficientArguments)
	}
}

impl<'a> TryFromArguments<'a> for Value {
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, state: &State) -> Result<Self> {
		Expression::try_from_arguments(arguments, state)
			.and_then(|expression| evaluate_expression(expression, state))
	}
}

impl<'a> TryFromArguments<'a> for u32 {
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, state: &State) -> Result<Self> {
		Value::try_from_arguments(arguments, state).map(u32::from)
	}
}

impl<'a> TryFromArguments<'a> for bool {
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, state: &State) -> Result<Self> {
		Value::try_from_arguments(arguments, state).map(bool::from)
	}
}

impl<'a> TryFromArguments<'a> for String {
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, state: &State) -> Result<Self> {
		Value::try_from_arguments(arguments, state).map(String::from)
	}
}

impl<'a, T> TryFromArguments<'a> for Option<T>
where
	T: TryFromArguments<'a>,
{
	fn try_from_arguments(arguments: &mut impl Arguments<'a>, state: &State) -> Result<Self> {
		match T::try_from_arguments(arguments, state) {
			Ok(value) => Ok(Some(value)),
			Err(Error::InsufficientArguments) => Ok(None),
			Err(error) => Err(error),
		}
	}
}

macro_rules! tuple_impl {
	($arg:ident $(, $args:ident)*) => {
		#[allow(non_camel_case_types)]
		impl<
			'a,
			$arg: TryFromArguments<'a>,
			$($args: TryFromArguments<'a>),*
		> TryFromArguments<'a> for ($arg, $($args),*) {
			fn try_from_arguments(
				arguments: &mut impl Arguments<'a>, state: &State
			) -> Result<Self> {
				Ok((
					$arg::try_from_arguments(arguments, state)?,
					$($args::try_from_arguments(arguments, state)?),*
				))
			}
		}

		tuple_impl!($($args),*);
	};

	() => {};
}

tuple_impl!(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
