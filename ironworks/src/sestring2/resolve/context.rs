use crate::sestring2::{error::Error, expression::Expression};

use super::{resolve::Resolve, value::Value};

#[derive(Debug)]
pub struct Context(());

impl Context {
	pub fn new() -> Self {
		Self(())
	}
}

impl Context {
	pub(super) fn evaluate(
		&self,
		expression: Expression,
		resolver: &mut impl Resolve,
	) -> Result<Value, Error> {
		let mut eval = |expr: Expression| self.evaluate(expr, resolver);

		let value = match expression {
			Expression::U32(value) => Value::U32(value),
			Expression::SeString(sestring) => {
				Value::String(resolver.resolve_sestring(sestring, self)?)
			}

			Expression::Millisecond => self.unknown(),
			Expression::Second => self.unknown(),
			Expression::Minute => self.unknown(),
			Expression::Hour => self.unknown(),
			Expression::Day => self.unknown(),
			Expression::Weekday => self.unknown(),
			Expression::Month => self.unknown(),
			Expression::Year => self.unknown(),

			// This is effectively a token/fallback value, the expression will be
			// caught before resolution by colour macros.
			Expression::StackColor => self.unknown(),

			Expression::LocalNumber(_expr) => self.unknown(),
			Expression::GlobalNumber(_expr) => self.unknown(),
			Expression::LocalString(_expr) => self.unknown(),
			Expression::GlobalString(_expr) => self.unknown(),

			Expression::Ge(left, right) => self.cmp(u32::ge, eval(*left)?, eval(*right)?),
			Expression::Gt(_, _) => todo!(),
			Expression::Le(_, _) => todo!(),
			Expression::Lt(_, _) => todo!(),
			Expression::Eq(left, right) => match self.eq(eval(*left)?, eval(*right)?) {
				true => Value::U32(1),
				false => Value::U32(0),
			},
			Expression::Ne(_, _) => todo!(),

			Expression::Unknown(_kind) => Value::Unknown,
		};

		Ok(value)
	}

	fn cmp(
		&self,
		cmp: impl for<'a, 'b> FnOnce(&'a u32, &'b u32) -> bool,
		left: Value,
		right: Value,
	) -> Value {
		// Unknown is treated as always-successful.
		if matches!(left, Value::Unknown) || matches!(right, Value::Unknown) {
			return Value::U32(1);
		}

		let left = u32::from(left);
		let right = u32::from(right);

		Value::U32(match cmp(&left, &right) {
			true => 1,
			false => 0,
		})
	}

	fn eq(&self, left: Value, right: Value) -> bool {
		match (left, right) {
			// Either side being UNKNOWN is truthy.
			(Value::Unknown, _) | (_, Value::Unknown) => true,
			// If both sides are strings, try to do a string comparison.
			(Value::String(left), Value::String(right)) => left == right,
			// Otherwise, coerce to u32 and compare.
			(left, right) => u32::from(left) == u32::from(right),
		}
	}

	fn unknown(&self) -> Value {
		Value::Unknown
	}
}
