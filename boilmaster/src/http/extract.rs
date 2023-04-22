use axum::{
	async_trait,
	extract::{
		rejection::{PathRejection, QueryRejection},
		FromRef, FromRequestParts,
	},
	http::request::Parts,
	RequestPartsExt,
};
use serde::Deserialize;

use crate::version::VersionKey;

use super::{error::Error, service};

#[derive(Deserialize)]
struct VersionQuery {
	version: Option<String>,
}

// TODO: Should I keep this directly on VersionKey or should it be on a wrapper newtype? Probably doesn't make a difference until we get to, like, multiserver architecture.
#[async_trait]
impl<S> FromRequestParts<S> for VersionKey
where
	S: Send + Sync,
	service::Version: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let Query(version_query) = parts
			.extract::<Query<VersionQuery>>()
			.await
			.map_err(|error| Error::Invalid(error.to_string()))?;

		let version = service::Version::from_ref(state);

		let version_name = version_query.version.as_deref();
		let version_key = version.resolve(version_name).ok_or_else(|| {
			Error::Invalid(format!(
				"unknown version \"{}\"",
				version_name.unwrap_or("(none)")
			))
		})?;

		Ok(version_key)
	}
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(Error))]
pub struct Path<T>(pub T);

impl From<PathRejection> for Error {
	fn from(value: PathRejection) -> Self {
		match value {
			PathRejection::FailedToDeserializePathParams(error) => Self::Invalid(error.body_text()),
			other => Self::Other(other.into()),
		}
	}
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(Error))]
pub struct Query<T>(pub T);

impl From<QueryRejection> for Error {
	fn from(value: QueryRejection) -> Self {
		match value {
			QueryRejection::FailedToDeserializeQueryString(error) => {
				Self::Invalid(error.body_text())
			}
			other => Self::Other(other.into()),
		}
	}
}
