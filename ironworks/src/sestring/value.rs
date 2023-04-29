use crate::{error::Result, Error, ErrorValue};

use super::{context::Context, expression::Expression};

// TODO: this should probably go in a seperate module
#[derive(Debug)]
pub enum Value {
	U32(u32),
	String(String),
}

impl TryFrom<Value> for u32 {
	type Error = Error;

	fn try_from(value: Value) -> Result<Self, Self::Error> {
		match value {
			Value::U32(value) => Ok(value),
			Value::String(_) => Err(Error::Invalid(
				ErrorValue::Other("SeString".into()),
				"cannot resolve string value to u32".into(),
			)),
		}
	}
}

impl TryFrom<Value> for String {
	type Error = Error;

	fn try_from(value: Value) -> Result<Self, Self::Error> {
		match value {
			Value::String(value) => Ok(value),
			Value::U32(_) => Err(Error::Invalid(
				ErrorValue::Other("SeString".into()),
				"cannot resolve u32 value to string".into(),
			)),
		}
	}
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
		T::resolve(self, context)
	}
}

pub trait FromArguments: Sized {
	fn resolve(arguments: &[Expression], context: &mut Context) -> Result<Self>;
}

// TODO: obv this should be macro'd
// TODO: should the error be generic?
impl<Arg1, Arg2, Arg3> FromArguments for (Arg1, Arg2, Arg3)
where
	Arg1: TryFrom<Value, Error = Error>,
	Arg2: TryFrom<Value, Error = Error>,
	Arg3: TryFrom<Value, Error = Error>,
{
	fn resolve(arguments: &[Expression], context: &mut Context) -> Result<Self> {
		let iter = &mut arguments.iter();
		Ok((
			resolve_argument(iter, context)?,
			resolve_argument(iter, context)?,
			resolve_argument(iter, context)?,
		))
	}
}

fn resolve_argument<'a, T>(
	iter: &mut impl Iterator<Item = &'a Expression>,
	context: &mut Context,
) -> Result<T>
where
	T: TryFrom<Value, Error = Error>,
{
	let expression = iter.next().ok_or_else(|| {
		Error::Invalid(
			ErrorValue::Other("SeString".into()),
			"insufficient arguments".into(),
		)
	})?;
	expression.resolve(context)
}
