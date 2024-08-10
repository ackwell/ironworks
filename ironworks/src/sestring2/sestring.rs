use binrw::{binread, helpers::until_exclusive};

use super::{cursor::SliceCursor, error::Error, expression::Expression, macro_kind::MacroKind};

// TODO: debug on this should probably be overwritten
#[binread]
#[derive(Debug)]
pub struct SeString {
	#[br(parse_with = until_exclusive(|&byte| byte == 0))]
	data: Vec<u8>,
}

impl SeString {
	pub fn iter(&self) -> PayloadIterator {
		PayloadIterator::new(&self.data)
	}
}

impl<'a> IntoIterator for &'a SeString {
	type Item = Result<Payload<'a>, Error>;
	type IntoIter = PayloadIterator<'a>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

const MACRO_START: u8 = 0x02;
const MACRO_END: u8 = 0x03;

#[derive(Debug)]
pub struct PayloadIterator<'a> {
	cursor: SliceCursor<'a>,
}

impl<'a> PayloadIterator<'a> {
	fn new(data: &'a [u8]) -> Self {
		Self {
			cursor: SliceCursor::new(data),
		}
	}
}

impl<'a> Iterator for PayloadIterator<'a> {
	type Item = Result<Payload<'a>, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		// EOF, stop iteration.
		if self.cursor.eof() {
			return None;
		}

		// Read the next payload.
		let payload = match read_payload(&mut self.cursor) {
			Err(error) => return Some(Err(error)),
			Ok(value) => value,
		};

		Some(Ok(payload))
	}
}

fn read_payload<'a>(cursor: &mut SliceCursor<'a>) -> Result<Payload<'a>, Error> {
	// Next byte is the start of a macro.
	if cursor.peek()? == MACRO_START {
		cursor.seek(1);

		let kind = MacroKind::from(cursor.next()?);

		// TODO: this will need some invalid error handling for non-u32
		let Expression::U32(length) = Expression::read(cursor)?;

		let body_length =
			usize::try_from(length).expect("Are you seriously running this on a 16bit system?");

		let body = cursor.take(body_length)?;

		if cursor.next()? != MACRO_END {
			return Err(Error::InvalidMacro);
		}

		return Ok(Payload::Macro(kind, body));
	}

	// Otherwise, read plain text until a macro is detected.
	let text_bytes = cursor.take_until(|&byte| byte == MACRO_START)?;

	Ok(Payload::Text(text_bytes))
}

// TODO: the variants should probably have dedicated types so i can add per-type trait impls people can use
#[derive(Debug, PartialEq)]
pub enum Payload<'a> {
	Text(&'a [u8]),
	Macro(MacroKind, &'a [u8]),
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
		assert_payloads(bytes, [Payload::Text(bytes)]);
	}

	#[test]
	fn macro_without_arguments() {
		assert_payloads(
			&[0x02, 0xFF, 0x01, 0x03],
			[Payload::Macro(MacroKind::Unknown(0xFF), &[])],
		)
	}

	#[test]
	fn macro_with_arguments() {
		// Two arguments of U32(0)
		assert_payloads(
			&[0x02, 0xFF, 0x03, 0x01, 0x01, 0x03],
			[Payload::Macro(MacroKind::Unknown(0xFF), &[0x01, 0x01])],
		)
	}

	#[test]
	fn mixed_payloads() {
		assert_payloads(
			b"before\x02\xFF\x02\x01\x03after",
			[
				Payload::Text(b"before"),
				Payload::Macro(MacroKind::Unknown(0xFF), &[0x01]),
				Payload::Text(b"after"),
			],
		)
	}

	fn assert_payloads<'a>(bytes: &'a [u8], payloads: impl IntoIterator<Item = Payload<'a>>) {
		let sestring = SeString {
			data: bytes.to_vec(),
		};
		iter_eq(
			sestring.iter(),
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
