use binrw::{binread, helpers::until_exclusive};

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

		let body_length = usize::try_from(length).expect("todo: error handling");

		let body = cursor.take(body_length)?;

		if cursor.next()? != MACRO_END {
			return Err(Error::InvalidMacro);
		}

		return Ok(Payload::Macro(kind));
	}

	// Otherwise, read plain text until a macro is detected.
	let text_bytes = cursor.take_until(|&byte| byte == MACRO_START)?;

	Ok(Payload::Text(text_bytes))
}

#[derive(Debug, PartialEq)]
pub enum Payload<'a> {
	Text(&'a [u8]),
	Macro(MacroKind),
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum MacroKind {
	NewLine,

	Unknown(u8),
}

impl From<u8> for MacroKind {
	fn from(value: u8) -> Self {
		match value {
			0x10 => Self::NewLine,

			other => Self::Unknown(other),
		}
	}
}

#[derive(Debug)]
pub enum Expression {
	U32(u32),
	// unknown? - will need non_exhaustive
}

impl Expression {
	// todo: probably need a lifetime on this?
	fn read(cursor: &mut SliceCursor) -> Result<Self, Error> {
		let kind = cursor.next()?;

		match kind {
			value @ 0x01..=0xCF => Ok(Self::U32(u32::from(value - 1))),
			other => todo!("unhandled expression kind {other:?}"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Error {
	UnexpectedEof,
	InvalidMacro,
}

// TODO: debug on this should probably be overwritten to (len,offset)
#[derive(Debug)]
struct SliceCursor<'a> {
	data: &'a [u8],
	offset: usize,
}

impl<'a> SliceCursor<'a> {
	fn new(data: &'a [u8]) -> Self {
		Self { data, offset: 0 }
	}

	fn eof(&self) -> bool {
		self.offset >= self.data.len()
	}

	fn peek(&self) -> Result<u8, Error> {
		match self.data.get(self.offset) {
			Some(&value) => Ok(value),
			None => Err(Error::UnexpectedEof),
		}
	}

	fn seek(&mut self, distance: usize) {
		self.offset += distance;
	}

	fn next(&mut self) -> Result<u8, Error> {
		let value = self.peek()?;
		self.seek(1);
		Ok(value)
	}

	fn take(&mut self, count: usize) -> Result<&'a [u8], Error> {
		let Some(value) = self.data.get(self.offset..(self.offset + count)) else {
			return Err(Error::UnexpectedEof);
		};
		self.seek(count);
		Ok(value)
	}

	fn take_until(&mut self, predicate: impl FnMut(&u8) -> bool) -> Result<&'a [u8], Error> {
		let length = match self.data.iter().skip(self.offset).position(predicate) {
			Some(position) => position,
			None => self.data.len() - self.offset,
		};

		self.take(length)
	}
}

#[cfg(test)]
mod test {
	use std::fmt;

	use super::*;

	#[test]
	fn plain_string() {
		let bytes = b"string";
		let sestring = SeString {
			data: bytes.to_vec(),
		};
		iter_eq(sestring.iter(), [Ok(Payload::Text(bytes))].into_iter());
	}

	#[test]
	fn expressionless_macro() {
		let bytes = &[0x02, 0x10, 0x01, 0x03];
		let sestring = SeString {
			data: bytes.to_vec(),
		};
		iter_eq(
			sestring.iter(),
			[Ok(Payload::Macro(MacroKind::NewLine))].into_iter(),
		)
	}

	#[test]
	fn macro_with_argument() {}

	#[test]
	fn mixed_payloads() {}

	#[test]
	fn nested_macros() {}

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
