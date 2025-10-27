//! Types and helpers for working with the SeString rich text format.

mod cursor;
mod error;
mod expression;
mod extract_text;
mod macro_kind;
mod macro_string;
mod payload;
mod plain_format;
mod sestr;
mod sestring;

pub mod format;

pub use {
	error::Error,
	expression::Expression,
	macro_kind::MacroKind,
	payload::{Expressions, MacroPayload, Payload, TextPayload},
	sestr::{Payloads, SeStr},
	sestring::SeString,
};
