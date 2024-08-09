use std::slice::SliceIndex;

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

// TODO: debug on this should probably be overwritten
#[derive(Debug)]
pub struct PayloadIterator<'a> {
	data: &'a [u8],
	offset: usize,
}

impl<'a> PayloadIterator<'a> {
	fn new(data: &'a [u8]) -> Self {
		Self { data, offset: 0 }
	}
}

impl<'a> Iterator for PayloadIterator<'a> {
	type Item = Result<Payload<'a>, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		// EOF, stop iteration.
		if self.offset >= self.data.len() {
			return None;
		}

		let (payload, read_bytes) = match read_payload(&self.data[self.offset..]) {
			// TODO: should we move the offset on error to prevent infinite loops? to the end? +1?
			Err(error) => return Some(Err(error)),
			Ok(value) => value,
		};

		self.offset += read_bytes;

		Some(Ok(payload))
	}
}

fn read_payload<'a>(bytes: &'a [u8]) -> Result<(Payload<'a>, usize), Error> {
	// Next byte is the start of a macro.
	if *get(bytes, 0)? == MACRO_START {
		let mut used = 1;

		let kind = MacroKind::from(*get(bytes, 1)?);
		used += 1;

		// TODO: this will need some invalid error handling for non-u32
		let (Expression::U32(length), length_length) = Expression::read(get(bytes, 2..)?)?;
		used += length_length;

		let body_length = usize::try_from(length).expect("todo: error handling");
		let start = 2 + length_length;
		let end = start + body_length;

		let body = get(bytes, start..end)?;
		used += body_length;

		if *get(bytes, end)? != MACRO_END {
			return Err(Error::InvalidMacro);
		}
		used += 1;

		return Ok((Payload::Macro(kind), used));
	}

	// Otherwise, read plain text until a macro or EOF is detected.
	let text_bytes = match bytes.iter().position(|&byte| byte == MACRO_START) {
		Some(position) => &bytes[..position],
		None => bytes,
	};

	Ok((Payload::Text(text_bytes), text_bytes.len()))
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
	// unknown?
}

impl Expression {
	// todo: can i avoid needing to return the length?
	fn read(slice: &[u8]) -> Result<(Self, usize), Error> {
		let foo = get(slice, 0)?;

		match foo {
			value @ 0x01..=0xCF => Ok((Self::U32(u32::from(value - 1)), 1)),
			other => todo!("unhandled expression kind {other:?}"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Error {
	UnexpectedEof,
	InvalidMacro,
}

fn get<T, I>(slice: &[T], index: I) -> Result<&I::Output, Error>
where
	I: SliceIndex<[T]>,
{
	match slice.get(index) {
		Some(output) => Ok(output),
		None => Err(Error::UnexpectedEof),
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
