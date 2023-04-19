use axum::{
	extract::State,
	headers::{authorization::Basic, Authorization},
	http::{header, Request, StatusCode},
	middleware::Next,
	response::{IntoResponse, Response},
	TypedHeader,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BasicAuth {
	username: String,
	password: String,
}

pub async fn basic_auth<B>(
	State(expected): State<BasicAuth>,
	authorization: Option<TypedHeader<Authorization<Basic>>>,
	request: Request<B>,
	next: Next<B>,
) -> Response {
	let authenticated = authorization.map_or(false, |TypedHeader(auth)| {
		auth.username() == expected.username && auth.password() == expected.password
	});

	match authenticated {
		true => next.run(request).await,
		false => {
			// TypedHeader seems to just... not have this? eh?
			(
				StatusCode::UNAUTHORIZED,
				[(
					header::WWW_AUTHENTICATE,
					"Basic realm=\"boilmaster\", charset=\"UTF-8\"",
				)],
			)
				.into_response()
		}
	}
}
