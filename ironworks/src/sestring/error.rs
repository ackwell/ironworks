/// Error states specific to SeString logic.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	/// End of source data was reached unexpectedly.
	#[error("unexpected EOF")]
	UnexpectedEof,

	/// An invalid text payload was read. Typically caused by invalid UTF8.
	#[error("invalid text payload")]
	InvalidText,

	/// An invalid macro payload was read.
	#[error("invalid macro payload")]
	InvalidMacro,

	/// An invalid expression was read.
	#[error("invalid expression")]
	InvalidExpression,

	/// Insufficient expressions were provided as arguments to a macro call.
	#[error("insufficient arguments for macro")]
	InsufficientArguments,

	/// Too many expressions were provided as argument to a macro call.
	#[error("too many arguments for macro")]
	TooManyArguments,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
