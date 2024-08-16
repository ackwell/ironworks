use std::str;

use super::{
	cursor::SliceCursor,
	error::{Error, Result},
	expression::Expression,
	macro_kind::MacroKind,
};

const MACRO_START: u8 = 0x02;
const MACRO_END: u8 = 0x03;

#[derive(Debug, PartialEq)]
pub enum Payload<'a> {
	Text(TextPayload<'a>),
	Macro(MacroPayload<'a>),
}

impl<'a> Payload<'a> {
	pub(super) fn read(cursor: &mut SliceCursor<'a>) -> Result<Self> {
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

		// Otherwise, read plain text until a macro is detected (or EOF).
		let text_bytes = cursor.take_until(|&byte| byte == MACRO_START)?;

		Ok(Payload::Text(TextPayload(text_bytes)))
	}
}

#[derive(Debug, PartialEq)]
pub struct TextPayload<'a>(&'a [u8]);

impl<'a> TextPayload<'a> {
	pub fn as_utf8(&self) -> Result<&'a str> {
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
	type Item = Result<Expression<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cursor.eof() {
			return None;
		}

		Some(Expression::read(&mut self.cursor))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn plain_string() {
		let bytes = b"string";
		assert_payload(bytes, Payload::Text(TextPayload(bytes)));
	}

	#[test]
	fn macro_without_arguments() {
		assert_payload(
			&[0x02, 0xFF, 0x01, 0x03],
			Payload::Macro(MacroPayload(MacroKind::Unknown(0xFF), &[])),
		)
	}

	#[test]
	fn macro_with_arguments() {
		// Two arguments of U32(0)
		assert_payload(
			&[0x02, 0xFF, 0x03, 0x01, 0x01, 0x03],
			Payload::Macro(MacroPayload(MacroKind::Unknown(0xFF), &[0x01, 0x01])),
		)
	}

	fn assert_payload<'a>(bytes: &'a [u8], expected: Payload<'a>) {
		let mut cursor = SliceCursor::new(bytes);
		let got = Payload::read(&mut cursor).expect("read should not fail");
		assert_eq!(got, expected);
	}
}