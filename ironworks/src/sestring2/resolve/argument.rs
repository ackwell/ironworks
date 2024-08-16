use crate::sestring2::{
	error::{Error, Result},
	expression::Expression,
	payload::Expressions,
};

use super::{context::Context, resolve::Resolve, value::Value};

impl<'a> Expressions<'a> {
	pub fn evaluate<T>(self, resolver: &mut impl Resolve, context: &Context) -> Result<T, Error>
	where
		T: TryFromArguments<'a>,
	{
		T::try_from_arguments(self, resolver, context)
	}
}

pub trait TryFromArguments<'a>: Sized {
	fn try_from_arguments(
		arguments: Expressions<'a>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self>;
}

// pub? this logic will need to be usable by external consumers... will it? they'll be using it via the args thingo
trait TryFromArgument<'a>: Sized {
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self>;
}

impl<'a> TryFromArgument<'a> for Expression<'a> {
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		_resolver: &mut impl Resolve,
		_context: &Context,
	) -> Result<Self> {
		argument.ok_or(Error::InsufficientArguments)
	}
}

impl TryFromArgument<'_> for Value {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self> {
		let expresssion = Expression::try_from_argument(argument, resolver, context)?;
		context.evaluate(expresssion, resolver)
	}
}

// note can't blanket impl these on from/into because it conflicts with the option<T> impl
impl TryFromArgument<'_> for u32 {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self> {
		let value = Value::try_from_argument(argument, resolver, context)?;
		Ok(value.into())
	}
}

impl TryFromArgument<'_> for String {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self> {
		let value = Value::try_from_argument(argument, resolver, context)?;
		Ok(value.into())
	}
}

impl<'a, T> TryFromArgument<'a> for Option<T>
where
	T: TryFromArgument<'a>,
{
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self> {
		Ok(match argument {
			None => None,
			some => Some(T::try_from_argument(some, resolver, context)?),
		})
	}
}

impl<'a, T> TryFromArguments<'a> for T
where
	T: TryFromArgument<'a>,
{
	fn try_from_arguments(
		mut arguments: Expressions<'a>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self> {
		let result = T::try_from_argument(arguments.next().transpose()?, resolver, context)?;
		check_exhausted(arguments)?;
		Ok(result)
	}
}

macro_rules! tuple_impl {
	($arg:ident $(, $args:ident)*) => {
		#[allow(non_camel_case_types)]
		impl<
			'a,
			$arg: TryFromArgument<'a>,
			$($args: TryFromArgument<'a>),*
		> TryFromArguments<'a> for ($arg, $($args),*) {
			fn try_from_arguments(
				mut arguments: Expressions<'a>,
				resolver: &mut impl Resolve,
				context: &Context,
			) -> Result<Self> {
				let result = (
					$arg::try_from_argument(arguments.next().transpose()?, resolver, context)?,
					$($args::try_from_argument(arguments.next().transpose()?, resolver, context)?),*
				);
				check_exhausted(arguments)?;
				Ok(result)
			}
		}

		tuple_impl!($($args),*);
	};

	() => {};
}

tuple_impl!(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);

fn check_exhausted(mut arguments: Expressions<'_>) -> Result<()> {
	match arguments.next() {
		None => Ok(()),
		Some(_) => Err(Error::TooManyArguments),
	}
}
