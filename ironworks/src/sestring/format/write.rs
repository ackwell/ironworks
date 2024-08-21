use std::fmt;

use crate::sestring::error::Result;

use super::style::{Color, ColorUsage, Style};

/// A trait for writing and formatting an [`SeString`](crate::sestring::SeString).
pub trait Write {
	/// Writes a string slice into this writer, returning whether the write succeeded.
	fn write_str(&mut self, str: &str) -> Result<()>;

	/// Sets a text style that will apply to subsequent string content. Default
	/// state is dependant on string usage, and is not encoded in strings
	/// themselves, however _most_ instances will default to `Edge` enabled, and
	/// all others disabled.
	///
	/// **Note for implementors**
	///
	/// Even if `Edge` is disabled, an outline should still be drawn if a
	/// [`ColorUsage::Edge`] is pushed.
	fn set_style(&mut self, style: Style, enabled: bool) -> Result<()> {
		let _ = (style, enabled);
		Ok(())
	}

	/// Pushes a color of the specified usage onto the stack. Stacks are
	/// per-usage, and start empty. Text should use the top-most colour in the
	/// stack for the given usage when rendered.
	fn push_color(&mut self, usage: ColorUsage, color: Color) -> Result<()> {
		let _ = (usage, color);
		Ok(())
	}

	/// Pops a color of the specified usage from the stack. See
	/// [`push_color`](Write::push_color) for more detail.
	fn pop_color(&mut self, usage: ColorUsage) -> Result<()> {
		let _ = usage;
		Ok(())
	}
}

/// Simple [`Write`] implementation.
///
/// This writer will collect all text into an internal string buffer. No
/// formatting of any kind is preserved. If formatting or alternate outputs are
/// required, a custom `Write` implementation can be used.
#[derive(Debug)]
pub struct PlainString(String);

impl PlainString {
	/// Construsts a new plain string writer.
	pub fn new() -> Self {
		Self(String::new())
	}
}

impl Write for PlainString {
	fn write_str(&mut self, str: &str) -> Result<()> {
		self.0.push_str(str);
		Ok(())
	}
}

impl From<PlainString> for String {
	fn from(value: PlainString) -> Self {
		value.0
	}
}

impl fmt::Display for PlainString {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(formatter)
	}
}
