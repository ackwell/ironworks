use std::io::{self, Cursor, Read, Seek};

use binrw::{BinRead, BinResult, ReadOptions};
use time::OffsetDateTime;

use crate::{error::Result, Error, ErrorValue};

use super::{
	context::Context,
	value::{TryFromValue, Value},
	SeString,
};

#[derive(Debug)]
pub enum Expression {
	// Inline values
	U32(u32),
	String(SeString),

	// PLaceholders
	UnknownD8, // used in a m:s:(this) setup, so presumably a sub-second value. is put in a two-digit zero-pad, so perhaps centiseconds?
	Second,    // maybe?
	Minute,
	Hour,
	Day,
	Weekday,
	Month,
	Year,

	// Expected to be placeholders
	// TODO: Look into this more
	UnknownEC,

	// Comparators
	Ge(Box<Expression>, Box<Expression>),
	Gt(Box<Expression>, Box<Expression>),
	Le(Box<Expression>, Box<Expression>),
	Lt(Box<Expression>, Box<Expression>),
	Eq(Box<Expression>, Box<Expression>),
	Ne(Box<Expression>, Box<Expression>),

	// Parameters, 1-indexed.
	IntegerParameter(Box<Expression>),
	PlayerParameter(Box<Expression>),
	StringParameter(Box<Expression>),
	ObjectParameter(Box<Expression>),
}

impl Expression {
	pub fn resolve<V>(&self, context: &mut Context) -> Result<V>
	where
		V: TryFromValue,
	{
		let value = match self {
			Self::U32(value) => Value::U32(*value),
			Self::String(string) => Value::String(string.resolve(context)?),

			// Given I have only educated guesses at what this does, and it's usage is relatively minimal, leaving as zero for now.
			Self::UnknownD8 => Value::U32(0),
			Self::Second => time(OffsetDateTime::second, context)?,
			Self::Minute => time(OffsetDateTime::minute, context)?,
			Self::Hour => time(OffsetDateTime::hour, context)?,
			Self::Day => time(OffsetDateTime::day, context)?,
			Self::Weekday => time(|dt| dt.weekday().number_from_sunday(), context)?,
			Self::Month => time(|dt| u8::from(dt.month()), context)?,
			Self::Year => time(OffsetDateTime::year, context)?,

			Self::UnknownEC => Value::U32(Value::UNKNOWN),

			Self::Ge(left, right) => compare(u32::ge, left, right, context)?,
			Self::Gt(left, right) => compare(u32::gt, left, right, context)?,
			Self::Le(left, right) => compare(u32::le, left, right, context)?,
			Self::Lt(left, right) => compare(u32::lt, left, right, context)?,
			Self::Eq(left, right) => compare(u32::eq, left, right, context)?,
			Self::Ne(left, right) => compare(u32::ne, left, right, context)?,

			Self::IntegerParameter(expression) => {
				let index = expression.resolve(context)?;
				Value::U32(context.integer_parameter(index))
			}
			Self::PlayerParameter(expression) => {
				let index = expression.resolve(context)?;
				Value::U32(context.player_parameter(index)?)
			}
			Self::StringParameter(expression) => {
				let index = expression.resolve(context)?;
				Value::String(context.string_parameter(index))
			}
			Self::ObjectParameter(expression) => {
				let index = expression.resolve(context)?;
				Value::String(context.object_parameter(index))
			}

			other => {
				return Err(Error::Invalid(
					ErrorValue::SeString,
					format!("unhandled expression kind {other:?}"),
				))
			}
		};

		V::try_from_value(Some(value))
	}
}

fn time<T>(get: impl FnOnce(OffsetDateTime) -> T, context: &mut Context) -> Result<Value>
where
	T: TryInto<u32>,
	T::Error: std::error::Error,
{
	let timestamp = context.time().ok_or_else(|| {
		Error::Invalid(
			ErrorValue::SeString,
			"time placeholder expression encountered, but no time has been set on the context"
				.into(),
		)
	})?;
	let datetime = OffsetDateTime::from_unix_timestamp(timestamp.into())
		.map_err(|error| Error::Invalid(ErrorValue::SeString, error.to_string()))?;

	Ok(Value::U32(get(datetime).try_into().unwrap()))
}

fn compare(
	cmp: impl for<'a, 'b> FnOnce(&'a u32, &'b u32) -> bool,
	left: &Expression,
	right: &Expression,
	context: &mut Context,
) -> Result<Value> {
	let left: u32 = left.resolve(context)?;
	let right: u32 = right.resolve(context)?;

	// Unknown is treated as always-successful.
	let success = left == Value::UNKNOWN || right == Value::UNKNOWN || cmp(&left, &right);

	Ok(Value::U32(match success {
		true => 1,
		false => 0,
	}))
}

impl Expression {
	// Utility for the commonly used read-expression-and-expect-it-to-be-a-number case.
	pub fn read_u32<R: Read + Seek>(reader: &mut R, options: &ReadOptions) -> BinResult<u32> {
		let expression = Self::read_options(reader, options, ())?;
		match expression {
			Self::U32(value) => Ok(value),
			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unexpected expression kind {other:?}, expected U32"),
			}),
		}
	}
}

impl BinRead for Expression {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let kind = u8::read_options(reader, options, ())?;

		let mut read_expr =
			|| -> BinResult<_> { Ok(Box::new(Expression::read_options(reader, options, ())?)) };

		let expression = match kind {
			0x01..=0xCF => Self::U32(u32::from(kind - 1)),

			0xD8 => Self::UnknownD8,
			0xD9 => Self::Second,
			0xDA => Self::Minute,
			0xDB => Self::Hour,
			0xDC => Self::Day,
			0xDD => Self::Weekday,
			0xDE => Self::Month,
			0xDF => Self::Year,

			0xE0 => Self::Ge(read_expr()?, read_expr()?),
			0xE1 => Self::Gt(read_expr()?, read_expr()?),
			0xE2 => Self::Le(read_expr()?, read_expr()?),
			0xE3 => Self::Lt(read_expr()?, read_expr()?),
			0xE4 => Self::Eq(read_expr()?, read_expr()?),
			0xE5 => Self::Ne(read_expr()?, read_expr()?),

			0xE8 => Self::IntegerParameter(read_expr()?),
			0xE9 => Self::PlayerParameter(read_expr()?),
			0xEA => Self::StringParameter(read_expr()?),
			0xEB => Self::ObjectParameter(read_expr()?),

			// ??? seems to be used as a "reset" marker for color/edgecolor?
			0xEC => Self::UnknownEC,

			0xF0..=0xFE => Self::U32(read_packed_u32(kind, reader, options)?),

			0xFF => Self::String(read_inline_sestring(reader, options)?),

			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unknown expression kind {other:#X}"),
			})?,
		};

		Ok(expression)
	}
}

fn read_packed_u32<R: Read + Seek>(
	kind: u8,
	reader: &mut R,
	options: &ReadOptions,
) -> BinResult<u32> {
	let flags = (kind + 1) & 0b1111;
	let mut bytes = [0; 4];
	for i in (0..=3).rev() {
		if (flags & (1 << i)) == 0 {
			continue;
		}
		bytes[i] = u8::read_options(reader, options, ())?;
	}
	Ok(u32::from_le_bytes(bytes))
}

fn read_inline_sestring<R: Read + Seek>(
	reader: &mut R,
	options: &ReadOptions,
) -> BinResult<SeString> {
	let length = Expression::read_u32(reader, options)?;

	// Using take_seekable here causes an infinte recursion on type resolution that I can't quite work out how to fix.
	let mut buffer = Cursor::new(Vec::with_capacity(length.try_into().unwrap()));
	io::copy(&mut reader.take(length.into()), &mut buffer)?;
	buffer.set_position(0);
	let sestring = SeString::read_options(&mut buffer, options, ())?;
	Ok(sestring)
}
