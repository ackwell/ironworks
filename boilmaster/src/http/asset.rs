use anyhow::Context;
use axum::{
	debug_handler,
	extract::{Path, Query, State},
	headers::ContentType,
	http::header,
	response::IntoResponse,
	routing::get,
	Router, TypedHeader,
};
use serde::Deserialize;

use crate::asset::Format;

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
	State(asset): State<service::Asset>,
) -> Result<impl IntoResponse> {
	let version_key = version
		.resolve(version_query.version.as_deref())
		.with_context(|| format!("unknown version {:?}", version_query.version))?;

	let bytes = asset.convert(version_key, &path, Format::Png)?;

	let filename = std::path::Path::new(&path).with_extension("png");
	let disposition = match filename.file_name().and_then(|name| name.to_str()) {
		Some(name) => format!("inline; filename=\"{name}\""),
		None => "inline".to_string(),
	};

	Ok((
		TypedHeader(ContentType::png()),
		// TypedHeader only has a really naive inline value with no ability to customise :/
		[(header::CONTENT_DISPOSITION, disposition)],
		bytes,
	))
}
