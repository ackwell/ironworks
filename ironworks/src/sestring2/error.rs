#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	#[error("unexpected EOF")]
	UnexpectedEof,

	#[error("invalid text payload")]
	InvalidText,

	#[error("invalid macro payload")]
	InvalidMacro,

	#[error("invalid expression")]
	InvalidExpression,

	#[error("insufficient arguments for macro")]
	InsufficientArguments,

	#[error("too many arguments for macro")]
	TooManyArguments,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
