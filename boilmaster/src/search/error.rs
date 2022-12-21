#[derive(thiserror::Error, Debug)]
pub enum SearchError {
	#[error("invalid field value on {}: could not coerce {} value to {}", .0.field, .0.got, .0.expected)]
	FieldType(FieldTypeError),

	/// The provided query cannot be mapped onto the sheet schema.
	#[error("query mismatch on {}: {}", .0.field, .0.reason)]
	QueryMismatch(MismatchError),

	/// The sheet schema in use does not map cleanly to the search index (and hence game data).
	#[error("schema mismatch on {}: {}", .0.field, .0.reason)]
	SchemaMismatch(MismatchError),

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
