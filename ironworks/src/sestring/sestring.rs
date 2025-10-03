use std::{borrow::Cow, fmt};

use crate::sestring::Expression;

use super::{cursor::SliceCursor, error::Result, format, payload::Payload};

/// Square Enix rich text format.
///
/// SeString data consistes of a combination of UTF8 text and "macros" that
/// perform operations ranging from text style and colour, to control flow and
/// data lookups. Individual sections of an SeString are represented by
/// [`Payload`]s.
///
/// This implementation does not eagerly parse the inner structures of the
/// string, as such it may represent an invalid state until queried further.
pub struct SeString<'a> {
	data: Cow<'a, [u8]>,
}

impl<'a> SeString<'a> {
	/// Constructs a new `SeString` from a byte slice.
	pub fn new(data: impl Into<Cow<'a, [u8]>>) -> Self {
		Self { data: data.into() }
	}

	/// Converts from `&'a SeString` to `SeString<'a>`. Useful for passing to methods
	/// that expect an owned value.
	pub fn as_ref(&'a self) -> SeString<'a> {
		Self::new(self.data.as_ref())
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.data
	}

	pub fn as_owned(&self) -> SeString<'static> {
		SeString {
			data: (*self.data).to_owned().into(),
		}
	}

	/// Returns an iterator over [`Payload`]s within this string.
	pub fn payloads(&'a self) -> Payloads<'a> {
		Payloads::new(&self.data)
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
	pub fn format(&self) -> Result<String> {
		let input = format::Input::new();
		let mut writer = format::PlainString::new();
		format::format(self.as_ref(), &input, &mut writer)?;
		Ok(writer.into())
	}

	pub fn macro_string(&self) -> Result<String> {
		self.macro_string_inner(false)
	}

	fn macro_string_inner(&self, inside_macro: bool) -> Result<String> {
		let mut result = String::new();
		for payload in self.payloads() {
			match payload? {
				Payload::Text(text) => {
					let escaped: &[char] = if inside_macro {
						&['<', '>', '[', ']', '(', ')', ',', '\\']
					} else {
						&['<', '>', '\\']
					};
					for c in text.as_utf8()?.chars() {
						if escaped.contains(&c) {
							result.push('\\');
						}
						result.push(c);
					}
				}
				Payload::Macro(macro_payload) => {
					result.push_str("<");
					result.push_str(&macro_payload.kind().name());
					let v = macro_payload
						.expressions()
						.map(|expr| {
							expr.and_then(|expr| {
								let mut r = String::new();
								Self::expr_macro_string(&expr, &mut r)?;
								Ok(r)
							})
						})
						.collect::<Result<Vec<_>, _>>()?;
					if !v.is_empty() {
						result.push_str("(");
						result.push_str(&v.join(", "));
						result.push_str(")");
					}
					result.push_str(">");
				}
			}
		}
		Ok(result)
	}

	fn expr_macro_string(expr: &Expression<'_>, result: &mut String) -> Result<()> {
		match expr {
			Expression::U32(value) => {
				result.push_str(&value.to_string());
			}
			Expression::SeString(sestring) => {
				result.push_str(&sestring.macro_string_inner(true)?);
			}
			Expression::Millisecond
			| Expression::Second
			| Expression::Minute
			| Expression::Hour
			| Expression::Day
			| Expression::Weekday
			| Expression::Month
			| Expression::Year
			| Expression::StackColor => {
				result.push_str(expr.name());
			}
			Expression::LocalNumber(e)
			| Expression::GlobalNumber(e)
			| Expression::LocalString(e)
			| Expression::GlobalString(e) => {
				result.push_str(expr.name());
				Self::expr_macro_string(e, result)?;
			}
			Expression::Ge(lhs, rhs)
			| Expression::Gt(lhs, rhs)
			| Expression::Le(lhs, rhs)
			| Expression::Lt(lhs, rhs)
			| Expression::Eq(lhs, rhs)
			| Expression::Ne(lhs, rhs) => {
				result.push_str("[");
				Self::expr_macro_string(lhs, result)?;
				result.push_str(expr.name());
				Self::expr_macro_string(rhs, result)?;
				result.push_str("]");
			}
			Expression::Unknown(value) => {
				result.push_str(&format!("unknown({value})"));
			}
		}
		Ok(())
	}
}

impl fmt::Display for SeString<'_> {
	/// Formats this string for display.
	///
	/// As SeString formatting is fallible, this implementation will fall back to
	/// a placeholder value when invalid input is provided. If error handling is
	/// needed for your use case, [`Self::format`] or the [`format`] module can be
	/// used instead.
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ret = if formatter.alternate() {
			self.macro_string()
		} else {
			self.format()
		};
		match ret {
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
		let sestring = SeString {
			data: Cow::Owned(bytes.to_vec()),
		};
		assert_eq!(sestring.payloads().count(), expected)
	}
}
