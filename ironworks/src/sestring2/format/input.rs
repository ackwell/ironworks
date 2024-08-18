use super::{
	style::{Color, ColorUsage},
	value::Value,
};

#[derive(Debug)]
pub struct Input {
	_private: (),
}

impl Input {
	pub fn new() -> Self {
		Self { _private: () }
	}

	// TODO
	pub fn local_parameter(&self, _id: u32) -> Value {
		Value::Unknown
	}

	// TODO
	pub fn global_parameter(&self, _id: u32) -> Value {
		Value::Unknown
	}

	// TODO
	pub fn color(&self, _usage: ColorUsage, _id: u32) -> Color {
		// magenta as a fallback
		Color {
			r: 255,
			g: 0,
			b: 255,
			a: 255,
		}
	}
}
