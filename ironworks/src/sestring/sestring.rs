use std::{borrow::Cow, fmt};

use super::{cursor::SliceCursor, error::Result, format, payload::Payload};

pub struct SeString<'a> {
	data: Cow<'a, [u8]>,
}

impl<'a> SeString<'a> {
	pub fn new(data: impl Into<Cow<'a, [u8]>>) -> Self {
		Self { data: data.into() }
	}

	pub fn as_ref(&'a self) -> SeString<'a> {
		Self::new(self.data.as_ref())
	}

	pub fn payloads(&'a self) -> Payloads<'a> {
		Payloads::new(&self.data)
	}

	pub fn format(&self) -> Result<String> {
		let input = format::Input::new();
		let mut writer = format::PlainString::new();
		format::format(self.as_ref(), &input, &mut writer)?;
		Ok(writer.into())
	}
}

impl fmt::Display for SeString<'_> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.format() {
			Ok(string) => string.fmt(formatter),
			Err(_error) => "invalid SeString".fmt(formatter),
		}
	}
}

impl fmt::Debug for SeString<'_> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter
			.debug_struct("SeString")
			.field("data", &format!("&[u8; {}]", self.data.len()))
			.finish()
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
	type Item = Result<Payload<'a>>;

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
		assert_eq!(sestring.payloads().count(), expected)
	}
}
