mod cursor;
mod error;
mod expression;
mod macro_kind;
mod payload;
mod sestring;

// TODO: is this how i wanna handle it?
pub mod format;

pub use {
	error::Error,
	expression::Expression,
	macro_kind::MacroKind,
	payload::{Expressions, MacroPayload, Payload, TextPayload},
	sestring::{Payloads, SeString},
};
