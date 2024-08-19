use std::fmt;

use super::style::{Color, ColorUsage, Style};

pub trait Write {
	// todo: should these return a result? very tempting to do so
	// write_str?
	fn write(&mut self, str: &str);

	// what about icons? sounds? does this really make sense as "write" still?

	// TODO: need to document the outline style behavior (pushed outline overrides a lack of outline)
	fn set_style(&mut self, style: Style, enabled: bool);

	fn push_color(&mut self, usage: ColorUsage, color: Color);

	fn pop_color(&mut self, usage: ColorUsage);
}

#[derive(Debug)]
pub struct PlainString(String);

impl PlainString {
	pub fn new() -> Self {
		Self(String::new())
	}
}

impl Write for PlainString {
	fn write(&mut self, str: &str) {
		self.0.push_str(str)
	}

	fn set_style(&mut self, _style: Style, _enabled: bool) {
		// noop
	}

	fn push_color(&mut self, _usage: ColorUsage, _color: Color) {
		// noop
	}

	fn pop_color(&mut self, _usage: ColorUsage) {
		// noop
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
