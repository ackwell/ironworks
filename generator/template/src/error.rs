use std::{error::Error, fmt};

use ironworks::excel::Field;

#[derive(Debug)]
pub struct PopulateError {
	inner: String,
}

impl fmt::Display for PopulateError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.inner)
	}
}

impl Error for PopulateError {}

impl From<ironworks::Error> for PopulateError {
	fn from(error: ironworks::Error) -> Self {
		Self {
			inner: error.to_string(),
		}
	}
}

impl From<Field> for PopulateError {
	// TODO: Would be nice to say what type was expected somehow, too.
	fn from(field: Field) -> Self {
		Self {
			inner: format!("Read unexpected field {field:?}"),
		}
	}
}
