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
