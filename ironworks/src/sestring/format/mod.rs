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
	runtime::{Gender, Player},
	style::{Color, ColorUsage, Style},
	value::Value,
	write::{PlainString, Write},
};
