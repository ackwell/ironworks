#[derive(thiserror::Error, Debug)]
pub enum SearchError {
	#[error(transparent)]
	FieldType(FieldTypeError),

	// TODO: is it worth disambigurating between "sheet schema did not match search schema" and "sheet schema did not match exh schema"? Technically speaking both signal user-provided error (exh means the game version and schema mismatch, search means the schema and the query mismatch).
	// i'm tempted to say that it is...
	#[error(transparent)]
	SchemaMismatch(SchemaMismatchError),

	#[error(transparent)]
	Failure(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("invalid field value on {field}: could not coerce {got} value to {expected}")]
pub struct FieldTypeError {
	pub(super) field: String,
	pub(super) expected: String,
	pub(super) got: String,
}

#[derive(thiserror::Error, Debug)]
#[error("schema mismatch on {field}: {reason}")]
pub struct SchemaMismatchError {
	pub(super) field: String,
	pub(super) reason: String,
}
