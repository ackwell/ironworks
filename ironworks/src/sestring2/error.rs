#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	#[error("unexpected EOF")]
	UnexpectedEof,

	#[error("invalid macro payload")]
	InvalidMacro,
}
