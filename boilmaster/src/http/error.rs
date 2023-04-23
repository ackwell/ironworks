use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::{asset, schema, search};

#[derive(Serialize)]
pub struct ErrorResponse {
	pub code: u16,
	pub message: String,
}

// TODO Should probably be an "api error"?
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Not found: {0}")]
	NotFound(String),

	#[error("Invalid request: {0}")]
	Invalid(String),

	#[error("Internal server error.")]
	Other(#[from] anyhow::Error),
}

impl From<asset::Error> for Error {
	fn from(error: asset::Error) -> Self {
		use asset::Error as AE;
		match error {
			AE::NotFound(value) => Self::NotFound(value),
			AE::UnsupportedSource(_, _) | AE::InvalidConversion(_, _) | AE::UnknownFormat(_) => {
				Self::Invalid(error.to_string())
			}
			AE::Failure(inner) => Self::Other(inner),
		}
	}
}

impl From<schema::Error> for Error {
	fn from(error: schema::Error) -> Self {
		use schema::Error as SE;
		match error {
			SE::UnknownSource(_) | SE::InvalidVersion(_) => Self::Invalid(error.to_string()),
			SE::Failure(inner) => Self::Other(inner),
		}
	}
}

impl From<search::Error> for Error {
	fn from(error: search::Error) -> Self {
		use search::Error as SE;
		match error {
			SE::FieldType(_)
			| SE::MalformedQuery(_)
			| SE::QuerySchemaMismatch(_)
			| SE::QueryGameMismatch(_)
			| SE::SchemaGameMismatch(_) => Self::Invalid(error.to_string()),
			SE::Failure(inner) => Self::Other(inner),
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		// Log the full error for ISEs - we don't show this info anywhere else in case it contains something sensitive.
		if let Self::Other(ref error) = self {
			tracing::error!("{error:?}")
		}

		// TODO: INCREDIBLY IMPORTANT: work out how to worm IM_A_TEAPOT into this
		let status_code = match self {
			Self::NotFound(_) => StatusCode::NOT_FOUND,
			Self::Invalid(_) => StatusCode::BAD_REQUEST,
			Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
		};

		(
			status_code,
			Json(ErrorResponse {
				code: status_code.as_u16(),
				message: self.to_string(),
			}),
		)
			.into_response()
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
