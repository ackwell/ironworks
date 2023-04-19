use anyhow::Context;
use axum::{
	debug_handler,
	extract::{Path, Query, State},
	response::IntoResponse,
	routing::get,
	Router,
};
use serde::Deserialize;

use super::{error::Result, service};

pub fn router() -> Router<service::State> {
	Router::new().route("/*path", get(asset))
}

#[derive(Deserialize)]
struct VersionQuery {
	version: Option<String>,
}

#[debug_handler(state = service::State)]
async fn asset(
	Path(path): Path<String>,
	Query(version_query): Query<VersionQuery>,
	State(version): State<service::Version>,
	State(data): State<service::Data>,
) -> Result<impl IntoResponse> {
	let version_key = version
		.resolve(version_query.version.as_deref())
		.with_context(|| format!("unknown version {:?}", version_query.version))?;

	let data_version = data
		.version(version_key)
		.with_context(|| format!("data for {version_key} not ready"))?;

	let ironworks = data_version.ironworks();

	let file = ironworks.file::<Vec<u8>>(&path)?;

	Ok(file)
}
