use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{request::Parts, StatusCode},
	Json,
};
use serde::de::DeserializeOwned;

use super::error::ErrorResponse;

#[derive(Debug)]
pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
	T: DeserializeOwned + Send,
	S: Send + Sync,
{
	type Rejection = (StatusCode, Json<ErrorResponse>);

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		match axum::extract::Path::<T>::from_request_parts(parts, state).await {
			Ok(value) => Ok(Self(value.0)),
			Err(rejection) => Err((
				StatusCode::BAD_REQUEST,
				Json(ErrorResponse {
					code: StatusCode::BAD_REQUEST.as_u16(),
					message: rejection.to_string(),
				}),
			)),
		}
	}
}
