//! Types and helpers for working with the SeString rich text format.

mod cursor;
mod error;
mod expression;
mod macro_kind;
mod payload;
mod sestring;

pub mod format;

pub use {
	error::Error,
	expression::Expression,
	macro_kind::MacroKind,
	payload::{Expressions, MacroPayload, Payload, TextPayload},
	sestring::{Payloads, SeString},
};
