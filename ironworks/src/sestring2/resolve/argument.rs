use crate::sestring2::{error::Error, expression::Expression, payload::Expressions};

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
	) -> Result<Self, Error>;
}

// pub? this logic will need to be usable by external consumers... will it? they'll be using it via the args thingo
trait TryFromArgument<'a>: Sized {
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error>;
}

impl<'a> TryFromArgument<'a> for Expression<'a> {
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		_resolver: &mut impl Resolve,
		_context: &Context,
	) -> Result<Self, Error> {
		argument.ok_or_else(|| todo!("error type"))
	}
}

impl TryFromArgument<'_> for Value {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
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
	) -> Result<Self, Error> {
		let value = Value::try_from_argument(argument, resolver, context)?;
		Ok(value.into())
	}
}

impl TryFromArgument<'_> for String {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
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
	) -> Result<Self, Error> {
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
	) -> Result<Self, Error> {
		let result = T::try_from_argument(arguments.next().transpose()?, resolver, context)?;

		// todo: check exhausted
		if let Some(_) = arguments.next() {
			todo!("not exhausted")
		}

		Ok(result)
	}
}

// this will be implemented with a macro once i do it properly
impl<'a, Arg1: TryFromArgument<'a>, Arg2: TryFromArgument<'a>, Arg3: TryFromArgument<'a>>
	TryFromArguments<'a> for (Arg1, Arg2, Arg3)
{
	fn try_from_arguments(
		mut arguments: Expressions<'a>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		let result = (
			Arg1::try_from_argument(arguments.next().transpose()?, resolver, context)?,
			Arg2::try_from_argument(arguments.next().transpose()?, resolver, context)?,
			Arg3::try_from_argument(arguments.next().transpose()?, resolver, context)?,
		);

		// todo: check exhausted
		if let Some(_) = arguments.next() {
			todo!("not exhausted")
		}

		Ok(result)
	}
}
