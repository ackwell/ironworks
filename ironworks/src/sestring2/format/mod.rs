mod argument;
mod character;
mod control_flow;
mod excel;
mod expression;
mod format;
mod input;
mod number;
mod runtime;
mod style;
mod text;
mod time;
mod value;
mod write;

#[cfg(test)]
mod test;

pub use {
	format::format,
	input::Input,
	style::{Color, ColorUsage, Style},
	write::{PlainString, Write},
};
