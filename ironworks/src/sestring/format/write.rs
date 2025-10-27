use std::{fmt, ops::Deref};

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

pub type PlainString = PlainWriter<String>;

/// Simple [`Write`] implementation.
///
/// This writer will collect all text into an internal writer. No
/// formatting of any kind is preserved. If formatting or alternate outputs are
/// required, a custom `Write` implementation can be used.
#[derive(Debug, Default)]
pub struct PlainWriter<T: fmt::Write>(T);

impl<T: fmt::Write> PlainWriter<T> {
	/// Creates a new `PlainString` writer with the given inner writer.
	pub fn new(inner: T) -> Self {
		Self(inner)
	}

	/// Consumes this writer, returning the inner writer.
	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T: fmt::Write> From<T> for PlainWriter<T> {
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

impl<T: fmt::Write> Deref for PlainWriter<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: fmt::Write> Write for PlainWriter<T> {
	fn write_str(&mut self, str: &str) -> Result<()> {
		self.0
			.write_str(str)
			.map_err(|_| crate::sestring::error::Error::InvalidText)
	}
}

impl<T: fmt::Display + fmt::Write> fmt::Display for PlainWriter<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(formatter)
	}
}
