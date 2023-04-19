use axum::{middleware, Router};
use serde::Deserialize;

use crate::http::service;

use super::{
	auth::{basic_auth, BasicAuth},
	version, versions,
};

#[derive(Debug, Deserialize)]
pub struct Config {
	auth: BasicAuth,
}

pub fn router(config: Config) -> Router<service::State> {
	Router::new()
		.merge(versions::router())
		.merge(version::router())
		.layer(middleware::from_fn_with_state(config.auth, basic_auth))
}
