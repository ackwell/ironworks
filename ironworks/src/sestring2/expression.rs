use super::{
	cursor::SliceCursor,
	error::{Error, Result},
	sestring::SeString,
};

#[non_exhaustive]
#[derive(Debug)]
pub enum Expression<'a> {
	U32(u32),
	SeString(SeString<'a>),

	Millisecond,
	Second,
	Minute,
	Hour,
	Day,
	Weekday,
	Month,
	Year,

	StackColor,

	LocalNumber(Box<Expression<'a>>),
	GlobalNumber(Box<Expression<'a>>),
	LocalString(Box<Expression<'a>>),
	GlobalString(Box<Expression<'a>>),

	Ge(Box<Expression<'a>>, Box<Expression<'a>>),
	Gt(Box<Expression<'a>>, Box<Expression<'a>>),
	Le(Box<Expression<'a>>, Box<Expression<'a>>),
	Lt(Box<Expression<'a>>, Box<Expression<'a>>),
	Eq(Box<Expression<'a>>, Box<Expression<'a>>),
	Ne(Box<Expression<'a>>, Box<Expression<'a>>),

	Unknown(u8),
}

impl<'a> Expression<'a> {
	pub(super) fn read(cursor: &mut SliceCursor<'a>) -> Result<Self> {
		let kind = cursor.next()?;

		let mut read_inner = || Ok(Box::new(Expression::read(cursor)?));

		let expression = match kind {
			value @ 0x01..=0xCF => Self::U32(u32::from(value - 1)),

			0xD8 => Self::Millisecond,
			0xD9 => Self::Second,
			0xDA => Self::Minute,
			0xDB => Self::Hour,
			0xDC => Self::Day,
			0xDD => Self::Weekday,
			0xDE => Self::Month,
			0xDF => Self::Year,

			0xE0 => Self::Ge(read_inner()?, read_inner()?),
			0xE1 => Self::Gt(read_inner()?, read_inner()?),
			0xE2 => Self::Le(read_inner()?, read_inner()?),
			0xE3 => Self::Lt(read_inner()?, read_inner()?),
			0xE4 => Self::Eq(read_inner()?, read_inner()?),
			0xE5 => Self::Ne(read_inner()?, read_inner()?),

			0xE8 => Self::LocalNumber(read_inner()?),
			0xE9 => Self::GlobalNumber(read_inner()?),
			0xEA => Self::LocalString(read_inner()?),
			0xEB => Self::GlobalString(read_inner()?),

			0xEC => Self::StackColor,

			kind @ 0xF0..=0xFE => Self::U32(read_packed_u32(cursor, kind)?),

			0xFF => Self::SeString(read_inline_sestring(cursor)?),

			other => Self::Unknown(other),
		};

		Ok(expression)
	}
}

fn read_packed_u32(cursor: &mut SliceCursor, kind: u8) -> Result<u32> {
	let flags = (kind + 1) & 0b1111;
	let mut bytes = [0; 4];
	for i in (0..=3).rev() {
		if (flags & (1 << i)) == 0 {
			continue;
		}
		bytes[i] = cursor.next()?;
	}
	Ok(u32::from_le_bytes(bytes))
}

fn read_inline_sestring<'a>(cursor: &mut SliceCursor<'a>) -> Result<SeString<'a>> {
	let Expression::U32(length) = Expression::read(cursor)? else {
		return Err(Error::InvalidExpression);
	};
	let string_length = usize::try_from(length).unwrap();
	let string = SeString::from(cursor.take(string_length)?);
	Ok(string)
}

#[cfg(test)]
mod test {
	use super::*;

	macro_rules! assert_matches {
		($got:expr, $($expected:tt)+) => {
			match $got {
				$($expected)+ => (),
				ref got => panic!("assertion failed: got {:?}, expected {}", got, stringify!($($expected)+))
			}
		}
	}

	#[test]
	fn u32_simple() {
		assert_matches!(read(&[0x01]), Expression::U32(0));
		assert_matches!(read(&[0xB5]), Expression::U32(180));
	}

	#[test]
	fn u32_packed() {
		assert_matches!(read(&[0xF0, 0x34]), Expression::U32(52));
		assert_matches!(read(&[0xF5, 0x18, 0x59]), Expression::U32(1595648));
	}

	#[test]
	fn string() {
		let got = read(b"\xFF\x05test");
		let Expression::SeString(sestring) = got else {
			panic!("expected SeString(_), got {got:?}")
		};
		assert_eq!(sestring.payloads().count(), 1);
	}

	#[test]
	fn nullary() {
		assert_matches!(read(&[0xD8]), Expression::Millisecond);
		assert_matches!(read(&[0xEC]), Expression::StackColor);
	}

	#[test]
	fn unary() {
		let got = read(&[0xE9, 0x49]);
		let Expression::GlobalNumber(inner) = got else {
			panic!("expected GlobalNumber(_), got {got:?}");
		};
		assert_matches!(inner.as_ref(), Expression::U32(72));
	}

	#[test]
	fn binary() {
		let got = read(&[0xE0, 0x49, 0x65]);
		let Expression::Ge(one, two) = got else {
			panic!("expected Ge(_, _), got {got:?}");
		};
		assert_matches!(one.as_ref(), Expression::U32(72));
		assert_matches!(two.as_ref(), Expression::U32(100));
	}

	fn read<'a>(bytes: &'a [u8]) -> Expression<'a> {
		let mut cursor = SliceCursor::new(bytes);
		Expression::read(&mut cursor).expect("read should not fail")
	}
}
