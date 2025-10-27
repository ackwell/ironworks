use std::fmt::{self};

use crate::sestring::{
	extract_text::ExtractText, macro_string::MacroString, plain_format::PlainFormat,
	sestring::SeString,
};

use super::{cursor::SliceCursor, error::Result, payload::Payload};

/// Square Enix rich text format.
///
/// SeString data consistes of a combination of UTF8 text and "macros" that
/// perform operations ranging from text style and colour, to control flow and
/// data lookups. Individual sections of an SeString are represented by
/// [`Payload`]s.
///
/// This implementation does not eagerly parse the inner structures of the
/// string, as such it may represent an invalid state until queried further.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeStr(pub(super) [u8]);

impl SeStr {
	/// Coerces into an `SeStr` slice.
	pub fn new<S: AsRef<SeStr> + ?Sized>(data: &S) -> &SeStr {
		data.as_ref()
	}

	#[inline]
	pub(super) fn from_inner(inner: &[u8]) -> &SeStr {
		// SAFETY: SeStr is just a wrapper of [u8],
		// therefore converting &[u8] to &SeStr is safe.
		unsafe { &*(inner as *const [u8] as *const SeStr) }
	}

	#[inline]
	pub(super) fn from_inner_mut(inner: &mut [u8]) -> &mut SeStr {
		// SAFETY: SeStr is just a wrapper of [u8],
		// therefore converting &mut [u8] to &mut SeStr is safe.
		unsafe { &mut *(inner as *mut [u8] as *mut SeStr) }
	}

	pub fn to_se_string(&self) -> SeString {
		SeString(self.0.to_owned())
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Converts a <code>[Box]<[SeStr]></code> into an [`SeString`] without copying or allocating.
	#[must_use = "`self` will be dropped if the result is not used"]
	pub fn into_se_string(self: Box<Self>) -> SeString {
		let boxed = unsafe { Box::from_raw(Box::into_raw(self) as *mut [u8]) };
		SeString(boxed.into_vec())
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	/// Returns an iterator over [`Payload`]s within this string.
	pub fn payloads(&self) -> Payloads<'_> {
		Payloads::new(&self.0)
	}

	/// Attempts to format this SeString into a plain, unstyled UTF8 string, with
	/// no input data.
	///
	/// As SeStrings are lazily parsed, this process may fail on invalid input.
	/// If this is irrelevant to your use case, a [`fmt::Display`] implementation
	/// is provided that will fall back to a placeholder value in this case.
	///
	/// If more control over the inputs and styling is desired, the [`format`]
	/// module can be used directly.
	pub fn format(&self) -> PlainFormat<'_> {
		PlainFormat::new(self)
	}

	/// Formats this SeString to a human-readable string representation,
	/// including macros.
	pub fn macro_string(&self) -> MacroString<'_> {
		MacroString::new(self)
	}

	/// Extracts the plain text from this SeString, replacing certain macros
	/// with their corresponding characters.
	pub fn extract_text(&self, use_soft_hyphen: bool) -> ExtractText<'_> {
		ExtractText::new(self, use_soft_hyphen)
	}
}

impl fmt::Display for SeStr {
	/// Formats this string for display.
	///
	/// As SeString formatting is fallible, this implementation will fall back to
	/// a placeholder value when invalid input is provided. If error handling is
	/// needed for your use case, [`Self::format`] or the [`format`] module can be
	/// used instead.
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		if formatter.alternate() {
			self.macro_string().fmt(formatter)
		} else {
			self.format().fmt(formatter)
		}
	}
}

impl fmt::Debug for SeStr {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter
			.debug_tuple("SeString")
			.field(&self.macro_string().to_string())
			.finish()
	}
}

impl<'a> From<&'a [u8]> for &'a SeStr {
	fn from(data: &'a [u8]) -> &'a SeStr {
		SeStr::from_inner(data)
	}
}

impl ToOwned for SeStr {
	type Owned = SeString;

	fn to_owned(&self) -> SeString {
		self.to_se_string()
	}
}

/// Iterator over [`Payload`]s within an [`SeString`]. As payloads are read on
/// demand, iteration may fail if a payload is invalid.
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
		let sestring: &SeStr = bytes.into();
		assert_eq!(sestring.payloads().count(), expected)
	}
}
