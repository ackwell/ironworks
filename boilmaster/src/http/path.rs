use axum::{
	async_trait,
	extract::{FromRequest, RequestParts},
	http::StatusCode,
	Json,
};
use serde::de::DeserializeOwned;

use super::error::ErrorResponse;

pub struct Path<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Path<T>
where
	T: DeserializeOwned + Send,
	B: Send,
{
	type Rejection = (StatusCode, Json<ErrorResponse>);

	async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
		match axum::extract::Path::<T>::from_request(req).await {
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
