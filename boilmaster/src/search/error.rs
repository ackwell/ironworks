#[derive(thiserror::Error, Debug)]
pub enum SearchError {
	#[error("invalid field value on {}: could not coerce {} value to {}", .0.field, .0.got, .0.expected)]
	FieldType(FieldTypeError),

	#[error("malformed search query: {0}")]
	MalformedQuery(String),

	/// The provided query cannot be mapped onto the sheet schema.
	#[error("query <-> schema mismatch on {}: {}", .0.field, .0.reason)]
	QuerySchemaMismatch(MismatchError),

	/// The provided query cannot be normalized in terms of the game data.
	#[error("query <-> game mismatch on {}: {}", .0.field, .0.reason)]
	QueryGameMismatch(MismatchError),

	/// The sheet schema in use does not map cleanly to the search index (and hence game data).
	#[error("schema <-> game mismatch on {}: {}", .0.field, .0.reason)]
	SchemaGameMismatch(MismatchError),

	#[error(transparent)]
	Failure(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct FieldTypeError {
	pub(super) field: String,
	pub(super) expected: String,
	pub(super) got: String,
}

#[derive(Debug)]
pub struct MismatchError {
	pub(super) field: String,
	pub(super) reason: String,
}
