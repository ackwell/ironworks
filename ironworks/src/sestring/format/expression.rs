use time::OffsetDateTime;

use crate::sestring::{
	error::{Error, Result},
	expression::Expression,
	sestring::SeString,
};

use super::{
	format::{format_sestring, State},
	value::Value,
	write::Write,
};

pub fn evaluate_expression(expression: Expression, state: &State) -> Result<Value> {
	let input = state.input;
	let eval = |expr: Box<Expression>| evaluate_expression(*expr, state);

	let value = match expression {
		Expression::U32(value) => Value::U32(value),
		Expression::SeString(value) => evaluate_sestring(value, state)?,

		Expression::Millisecond => time(OffsetDateTime::millisecond, state)?,
		Expression::Second => time(OffsetDateTime::second, state)?,
		Expression::Minute => time(OffsetDateTime::minute, state)?,
		Expression::Hour => time(OffsetDateTime::hour, state)?,
		Expression::Day => time(OffsetDateTime::day, state)?,
		Expression::Weekday => time(|dt| dt.weekday().number_from_sunday(), state)?,
		Expression::Month => time(|dt| u8::from(dt.month()), state)?,
		Expression::Year => time(OffsetDateTime::year, state)?,

		Expression::StackColor => Value::Unknown,

		Expression::LocalNumber(inner) => u32(input.local_parameter(eval(inner)?.into())),
		Expression::GlobalNumber(inner) => u32(input.global_parameter(eval(inner)?.into())),
		Expression::LocalString(inner) => str(input.local_parameter(eval(inner)?.into())),
		Expression::GlobalString(inner) => str(input.global_parameter(eval(inner)?.into())),

		Expression::Ge(left, right) => cmp(u32::ge, eval(left)?, eval(right)?),
		Expression::Gt(left, right) => cmp(u32::gt, eval(left)?, eval(right)?),
		Expression::Le(left, right) => cmp(u32::le, eval(left)?, eval(right)?),
		Expression::Lt(left, right) => cmp(u32::lt, eval(left)?, eval(right)?),
		Expression::Eq(left, right) => eq(eval(left)?, eval(right)?),
		Expression::Ne(left, right) => !eq(eval(left)?, eval(right)?),

		Expression::Unknown(_unk) => Value::Unknown,
	};

	Ok(value)
}

fn evaluate_sestring(sestring: SeString, state: &State) -> Result<Value> {
	let mut writer = EvaluationWriter(String::new());
	let mut state = State {
		input: state.input,
		writer: &mut writer,
		time: state.time,
	};
	format_sestring(sestring, &mut state)?;
	Ok(Value::String(writer.0))
}

struct EvaluationWriter(String);
impl Write for EvaluationWriter {
	fn write_str(&mut self, str: &str) -> Result<()> {
		self.0.push_str(str);
		Ok(())
	}

	fn set_style(&mut self, _style: super::Style, _enabled: bool) -> Result<()> {
		// noop, ref. LogMessage@German 4148:0
		Ok(())
	}

	fn push_color(&mut self, _usage: super::ColorUsage, _color: super::Color) -> Result<()> {
		unimplemented!()
	}

	fn pop_color(&mut self, _usage: super::ColorUsage) -> Result<()> {
		unimplemented!()
	}
}

fn time<T>(get: impl FnOnce(OffsetDateTime) -> T, state: &State) -> Result<Value>
where
	T: TryInto<u32>,
	T::Error: std::error::Error,
{
	let datetime = OffsetDateTime::from_unix_timestamp(state.time.into())
		.map_err(|_err| Error::InvalidExpression)?;

	Ok(Value::U32(
		get(datetime)
			.try_into()
			.expect("time conversion should not fail"),
	))
}

fn u32(value: Value) -> Value {
	match value {
		Value::Unknown => Value::Unknown,
		other => Value::U32(other.into()),
	}
}

fn str(value: Value) -> Value {
	match value {
		Value::Unknown => Value::Unknown,
		other => Value::String(other.into()),
	}
}

fn cmp(cmp: impl for<'a, 'b> FnOnce(&'a u32, &'b u32) -> bool, left: Value, right: Value) -> Value {
	// Unknown is treated as always-successful.
	if matches!(left, Value::Unknown) || matches!(right, Value::Unknown) {
		return Value::TRUE;
	}

	let left = u32::from(left);
	let right = u32::from(right);

	cmp(&left, &right).into()
}

fn eq(left: Value, right: Value) -> Value {
	match (left, right) {
		// Either side being unknown is truthy.
		(Value::Unknown, _) | (_, Value::Unknown) => Value::TRUE,
		// If both sides are strings, attempt string comparison.
		(Value::String(left), Value::String(right)) => (left == right).into(),
		// Otherwise, compare coerced u32.
		(left, right) => (u32::from(left) == u32::from(right)).into(),
	}
}
