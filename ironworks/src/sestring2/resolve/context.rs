use crate::sestring2::{error::Result, expression::Expression};

use super::{resolve::Resolve, value::Value};

// TODO: i think this needs to be split into a read-only "parameters" type, and context, which is mutable. context consumed by reading, but parameters are reusable.
#[derive(Debug)]
pub struct Context(());

impl Context {
	pub fn new() -> Self {
		Self(())
	}
}

impl Context {
	pub(super) fn set_time(&mut self, timestamp: u32) {}

	pub(super) fn evaluate(
		&mut self,
		expression: Expression,
		resolver: &mut impl Resolve,
	) -> Result<Value> {
		let mut eval = |expr: Expression| self.evaluate(expr, resolver);

		let value = match expression {
			Expression::U32(value) => Value::U32(value),
			Expression::SeString(sestring) => {
				Value::String(resolver.resolve_sestring(sestring, self)?)
			}

			Expression::Millisecond => Value::Unknown,
			Expression::Second => Value::Unknown,
			Expression::Minute => Value::Unknown,
			Expression::Hour => Value::Unknown,
			Expression::Day => Value::Unknown,
			Expression::Weekday => Value::Unknown,
			Expression::Month => Value::Unknown,
			Expression::Year => Value::Unknown,

			// This is effectively a token/fallback value, the expression will be
			// caught before resolution by colour macros.
			Expression::StackColor => Value::Unknown,

			Expression::LocalNumber(_expr) => Value::Unknown,
			Expression::GlobalNumber(_expr) => Value::Unknown,
			Expression::LocalString(_expr) => Value::Unknown,
			Expression::GlobalString(_expr) => Value::Unknown,

			Expression::Ge(left, right) => cmp(u32::ge, eval(*left)?, eval(*right)?),
			Expression::Gt(left, right) => cmp(u32::gt, eval(*left)?, eval(*right)?),
			Expression::Le(left, right) => cmp(u32::le, eval(*left)?, eval(*right)?),
			Expression::Lt(left, right) => cmp(u32::lt, eval(*left)?, eval(*right)?),
			Expression::Eq(left, right) => match eq(eval(*left)?, eval(*right)?) {
				true => Value::U32(1),
				false => Value::U32(0),
			},
			Expression::Ne(_, _) => todo!(),

			Expression::Unknown(_kind) => Value::Unknown,
		};

		Ok(value)
	}
}

fn cmp(cmp: impl for<'a, 'b> FnOnce(&'a u32, &'b u32) -> bool, left: Value, right: Value) -> Value {
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

fn eq(left: Value, right: Value) -> bool {
	match (left, right) {
		// Either side being UNKNOWN is truthy.
		(Value::Unknown, _) | (_, Value::Unknown) => true,
		// If both sides are strings, try to do a string comparison.
		(Value::String(left), Value::String(right)) => left == right,
		// Otherwise, coerce to u32 and compare.
		(left, right) => u32::from(left) == u32::from(right),
	}
}
