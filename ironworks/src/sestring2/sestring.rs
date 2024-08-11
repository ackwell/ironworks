use std::{borrow::Cow, str};

use binrw::{binread, helpers::until_exclusive};

use super::{cursor::SliceCursor, error::Error, expression::Expression, macro_kind::MacroKind};

// TODO: debug on this should probably be overwritten
#[binread]
#[derive(Debug)]
pub struct SeString<'a> {
	#[br(parse_with = until_exclusive(|&byte| byte == 0))]
	data: Cow<'a, [u8]>,
}

impl<'a> From<&'a [u8]> for SeString<'a> {
	fn from(value: &'a [u8]) -> Self {
		Self {
			data: Cow::Borrowed(value),
		}
	}
}

impl SeString<'_> {
	pub fn payloads<'a>(&'a self) -> Payloads<'a> {
		Payloads::new(&self.data)
	}
}

#[derive(Debug)]
pub struct Payloads<'a> {
	cursor: SliceCursor<'a>,
}

impl<'a> Payloads<'a> {
	fn new(data: &'a [u8]) -> Self {
		Self {
			cursor: SliceCursor::new(data),
		}
	}
}

impl<'a> Iterator for Payloads<'a> {
	type Item = Result<Payload<'a>, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		// EOF, stop iteration.
		if self.cursor.eof() {
			return None;
		}

		// Read the next payload.
		Some(read_payload(&mut self.cursor))
	}
}

const MACRO_START: u8 = 0x02;
const MACRO_END: u8 = 0x03;

fn read_payload<'a>(cursor: &mut SliceCursor<'a>) -> Result<Payload<'a>, Error> {
	// Next byte is the start of a macro.
	if cursor.peek()? == MACRO_START {
		cursor.seek(1);

		let kind = MacroKind::from(cursor.next()?);

		let Expression::U32(length) = Expression::read(cursor)? else {
			return Err(Error::InvalidMacro);
		};

		let body_length =
			usize::try_from(length).expect("Are you seriously running this on a 16bit system?");

		let body = cursor.take(body_length)?;

		if cursor.next()? != MACRO_END {
			return Err(Error::InvalidMacro);
		}

		return Ok(Payload::Macro(MacroPayload(kind, body)));
	}

	// Otherwise, read plain text until a macro is detected.
	let text_bytes = cursor.take_until(|&byte| byte == MACRO_START)?;

	Ok(Payload::Text(TextPayload(text_bytes)))
}

#[derive(Debug, PartialEq)]
pub enum Payload<'a> {
	Text(TextPayload<'a>),
	Macro(MacroPayload<'a>),
}

#[derive(Debug, PartialEq)]
pub struct TextPayload<'a>(&'a [u8]);

impl<'a> TextPayload<'a> {
	pub fn as_utf8(&self) -> Result<&'a str, Error> {
		str::from_utf8(&self.0).map_err(|_error| Error::InvalidText)
	}
}

#[derive(Debug, PartialEq)]
pub struct MacroPayload<'a>(MacroKind, &'a [u8]);

impl<'a> MacroPayload<'a> {
	pub fn kind(&self) -> MacroKind {
		self.0
	}

	pub fn expressions(&self) -> Expressions<'a> {
		Expressions::new(self.1)
	}
}

#[derive(Debug)]
pub struct Expressions<'a> {
	cursor: SliceCursor<'a>,
}

impl<'a> Expressions<'a> {
	fn new(data: &'a [u8]) -> Self {
		Self {
			cursor: SliceCursor::new(data),
		}
	}
}

impl<'a> Iterator for Expressions<'a> {
	type Item = Result<Expression<'a>, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cursor.eof() {
			return None;
		}

		Some(Expression::read(&mut self.cursor))
	}
}

#[cfg(test)]
mod test {
	use std::fmt;

	use super::*;

	#[test]
	fn empty_string() {
		assert_payloads(&[], []);
	}

	#[test]
	fn plain_string() {
		let bytes = b"string";
		assert_payloads(bytes, [Payload::Text(TextPayload(bytes))]);
	}

	#[test]
	fn macro_without_arguments() {
		assert_payloads(
			&[0x02, 0xFF, 0x01, 0x03],
			[Payload::Macro(MacroPayload(MacroKind::Unknown(0xFF), &[]))],
		)
	}

	#[test]
	fn macro_with_arguments() {
		// Two arguments of U32(0)
		assert_payloads(
			&[0x02, 0xFF, 0x03, 0x01, 0x01, 0x03],
			[Payload::Macro(MacroPayload(
				MacroKind::Unknown(0xFF),
				&[0x01, 0x01],
			))],
		)
	}

	#[test]
	fn mixed_payloads() {
		assert_payloads(
			b"before\x02\xFF\x02\x01\x03after",
			[
				Payload::Text(TextPayload(b"before")),
				Payload::Macro(MacroPayload(MacroKind::Unknown(0xFF), &[0x01])),
				Payload::Text(TextPayload(b"after")),
			],
		)
	}

	fn assert_payloads<'a>(bytes: &'a [u8], payloads: impl IntoIterator<Item = Payload<'a>>) {
		let sestring = SeString {
			data: Cow::Owned(bytes.to_vec()),
		};
		iter_eq(
			sestring.payloads(),
			payloads.into_iter().map(|payload| Ok(payload)),
		)
	}

	// Yoinked from itertools.
	fn iter_eq<T>(mut a: impl Iterator<Item = T>, mut b: impl Iterator<Item = T>)
	where
		T: fmt::Debug + PartialEq,
	{
		loop {
			match (a.next(), b.next()) {
				(None, None) => return,
				(a_next, b_next) => {
					let equal = match (&a_next, &b_next) {
						(Some(a_val), Some(b_val)) => a_val == b_val,
						_ => false,
					};
					assert!(equal, "{a_next:?} != {b_next:?}")
				}
			}
		}
	}
}
