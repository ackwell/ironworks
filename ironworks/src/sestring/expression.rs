use std::{
	borrow::Cow,
	io::{self, Cursor, Read, Seek},
};

use binrw::{BinRead, BinResult, ReadOptions};

use super::{context::Context, SeString};

#[derive(Debug)]
pub enum Expression {
	// Inline values
	U32(u32),
	String(SeString),

	// PLaceholders
	UnkD8, // used in a m:s:(this) setup, so presumably a sub-second value. is put in a two-digit zero-pad, so perhaps centiseconds?
	Second, // maybe?
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
	Gte(Box<Expression>, Box<Expression>),
	Gt(Box<Expression>, Box<Expression>),
	Lte(Box<Expression>, Box<Expression>),
	Lt(Box<Expression>, Box<Expression>),
	Eq(Box<Expression>, Box<Expression>),
	Ne(Box<Expression>, Box<Expression>),

	// Parameters
	IntegerParameter(Box<Expression>),
	PlayerParameter(Box<Expression>),
	StringParameter(Box<Expression>),
	ObjectParameter(Box<Expression>),
}

pub enum Value<'a> {
	U32(u32),
	String(Cow<'a, str>),
}

impl Expression {
	pub fn resolve(&self, context: &mut Context) -> Value {
		match self {
			Self::U32(value) => Value::U32(*value),
			Self::String(string) => Value::String(string.resolve(context)),
			other => todo!("resolve expression kind {other:?}"),
		}
	}
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

			0xD8 => Self::UnkD8,
			0xD9 => Self::Second,
			0xDA => Self::Minute,
			0xDB => Self::Hour,
			0xDC => Self::Day,
			0xDD => Self::Weekday,
			0xDE => Self::Month,
			0xDF => Self::Year,

			0xE0 => Self::Gte(read_expr()?, read_expr()?),
			0xE1 => Self::Gt(read_expr()?, read_expr()?),
			0xE2 => Self::Lte(read_expr()?, read_expr()?),
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
