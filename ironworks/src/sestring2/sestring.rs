use std::borrow::Cow;

use binrw::{binread, helpers::until_exclusive};

use super::{cursor::SliceCursor, error::Error, payload::Payload};

// TODO: debug on this should probably be overwritten
#[binread]
#[derive(Debug)]
pub struct SeString<'a> {
	// not convinced by having binread in this
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
		Some(Payload::read(&mut self.cursor))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn empty_string() {
		assert_count(&[], 0);
	}

	#[test]
	fn mixed_payloads() {
		assert_count(b"before\x02\xFF\x02\x01\x03after", 3)
	}

	#[test]
	fn nested_macros() {
		let bytes = &[
			0x02, // start macro
			0xFF, // macro type
			0x07, // macro body length = 7
			0xFF, // inline string expression
			0x05, // inline string length = 4
			0x02, // start macro
			0xFF, // macro type
			0x01, // macro body length = 0,
			0x03, // end macro
			0x03, // end macro
		];
		assert_count(bytes, 1);
	}

	fn assert_count<'a>(bytes: &'a [u8], expected: usize) {
		let sestring = SeString {
			data: Cow::Owned(bytes.to_vec()),
		};
		println!("{:?}", sestring.payloads().collect::<Vec<_>>());
		assert_eq!(sestring.payloads().count(), expected)
	}
}
