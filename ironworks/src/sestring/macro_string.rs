use std::{
	fmt::{self, Display},
	sync::LazyLock,
};

use super::error::Result;
use crate::sestring::{Expression, Payload, SeStr};

pub struct MacroString<'a>(&'a SeStr);

impl MacroString<'_> {
	pub(super) fn new(str: &SeStr) -> MacroString<'_> {
		MacroString(str)
	}

	fn fmt_string(
		str: &SeStr,
		inside_macro: bool,
		formatter: &mut fmt::Formatter<'_>,
	) -> Result<()> {
		for payload in str.payloads() {
			match payload? {
				Payload::Text(text) => {
					write_escaped(formatter, text.as_utf8()?, inside_macro)?;
				}
				Payload::Macro(macro_payload) => {
					formatter.write_str("<")?;
					formatter.write_str(&macro_payload.kind().name())?;
					let expressions = macro_payload.expressions();
					let mut is_empty = true;
					for (i, expr) in expressions.enumerate() {
						formatter.write_str(if i == 0 {
							is_empty = false;
							"("
						} else {
							","
						})?;
						Self::fmt_expression(&expr?, formatter)?;
					}
					if !is_empty {
						formatter.write_str(")")?;
					}
					formatter.write_str(">")?;
				}
			}
		}
		Ok(())
	}

	fn fmt_expression(expr: &Expression<'_>, formatter: &mut fmt::Formatter<'_>) -> Result<()> {
		match expr {
			Expression::U32(value) => {
				value.fmt(formatter)?;
			}
			Expression::SeString(sestring) => {
				Self::fmt_string(sestring, true, formatter)?;
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
				formatter.write_str(expr.name())?;
			}
			Expression::LocalNumber(e)
			| Expression::GlobalNumber(e)
			| Expression::LocalString(e)
			| Expression::GlobalString(e) => {
				formatter.write_str(expr.name())?;
				Self::fmt_expression(e, formatter)?;
			}
			Expression::Ge(lhs, rhs)
			| Expression::Gt(lhs, rhs)
			| Expression::Le(lhs, rhs)
			| Expression::Lt(lhs, rhs)
			| Expression::Eq(lhs, rhs)
			| Expression::Ne(lhs, rhs) => {
				formatter.write_str("[")?;
				Self::fmt_expression(lhs, formatter)?;
				formatter.write_str(expr.name())?;
				Self::fmt_expression(rhs, formatter)?;
				formatter.write_str("]")?;
			}
			Expression::Unknown(value) => {
				write!(formatter, "unknown({value})")?;
			}
		}
		Ok(())
	}
}

impl Display for MacroString<'_> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		Self::fmt_string(self.0, false, formatter).map_err(|_| fmt::Error)
	}
}

const ESCAPED_CHARS_IN_MACRO: &[u8] = &[b'<', b'>', b'[', b']', b'(', b')', b',', b'\\'];
const ESCAPED_CHARS: [u8; 3] = [b'<', b'>', b'\\'];
const ESCAPE_CHAR: u8 = b'\\';

static ESC_TABLE: LazyLock<[bool; 256]> = LazyLock::new(|| {
	let mut t = [false; 256];
	for &x in ESCAPED_CHARS_IN_MACRO {
		t[x as usize] = true;
	}
	t
});

fn write_escaped(f: &mut fmt::Formatter<'_>, str: &str, inside_macro: bool) -> fmt::Result {
	let s = str;
	let b = s.as_bytes();

	if !inside_macro {
		let (n1, n2, n3) = (ESCAPED_CHARS[0], ESCAPED_CHARS[1], ESCAPED_CHARS[2]);

		let mut start = 0usize;
		while start < b.len() {
			if let Some(rel) = memchr::memchr3(n1, n2, n3, &b[start..]) {
				let pos = start + rel;
				if pos != start {
					f.write_str(&s[start..pos])?;
				}
				// write backslash + escaped byte (both ASCII => safe single-bytes)
				let esc = [ESCAPE_CHAR, b[pos]];
				// SAFETY: both bytes are ASCII, so valid UTF-8
				f.write_str(unsafe { std::str::from_utf8_unchecked(&esc) })?;
				start = pos + 1;
			} else {
				f.write_str(&s[start..])?;
				break;
			}
		}
		return Ok(());
	}

	let table = &*ESC_TABLE;
	let mut start = 0usize;
	while start < b.len() {
		let mut pos = start;
		while pos < b.len() && !table[b[pos] as usize] {
			pos += 1;
		}

		if pos != start {
			f.write_str(&s[start..pos])?;
		}
		if pos == b.len() {
			break;
		}
		// write backslash + escaped byte (both ASCII => safe single-bytes)
		let esc = [ESCAPE_CHAR, b[pos]];
		// SAFETY: both bytes are ASCII, so valid UTF-8
		f.write_str(unsafe { std::str::from_utf8_unchecked(&esc) })?;
		start = pos + 1;
	}
	Ok(())
}
