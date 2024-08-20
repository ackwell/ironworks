use std::fmt;

use crate::sestring2::error::Result;

use super::style::{Color, ColorUsage, Style};

pub trait Write {
	fn write_str(&mut self, str: &str) -> Result<()>;

	// TODO: need to document the edge style behavior (pushed edge overrides a lack of edge)
	fn set_style(&mut self, style: Style, enabled: bool) -> Result<()> {
		let _ = (style, enabled);
		Ok(())
	}

	fn push_color(&mut self, usage: ColorUsage, color: Color) -> Result<()> {
		let _ = (usage, color);
		Ok(())
	}

	fn pop_color(&mut self, usage: ColorUsage) -> Result<()> {
		let _ = usage;
		Ok(())
	}
}

#[derive(Debug)]
pub struct PlainString(String);

impl PlainString {
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
