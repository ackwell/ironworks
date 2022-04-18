pub struct PopulateError {
	inner: String,
}

impl std::string::ToString for PopulateError {
	fn to_string(&self) -> String {
		self.inner.clone()
	}
}

impl std::convert::From<ironworks::Error> for PopulateError {
	fn from(error: ironworks::Error) -> Self {
		Self {
			inner: error.to_string(),
		}
	}
}

impl std::convert::From<ironworks::excel::Field> for PopulateError {
	// TODO: Would be nice to say what type was expected somehow, too.
	fn from(field: ironworks::excel::Field) -> Self {
		Self {
			inner: format!("Read unexpected field {field:?}"),
		}
	}
}
