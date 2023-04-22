use axum::{
	debug_handler, extract::State, headers::ContentType, http::header, response::IntoResponse,
	routing::get, Router, TypedHeader,
};
use serde::Deserialize;

use crate::{asset::Format, version::VersionKey};

use super::{
	error::Result,
	extract::{Path, Query},
	service,
};

pub fn router() -> Router<service::State> {
	Router::new().route("/*path", get(asset))
}

#[derive(Deserialize)]
struct FormatQuery {
	format: Format,
}

#[debug_handler(state = service::State)]
async fn asset(
	Path(path): Path<String>,
	version_key: VersionKey,
	Query(format_query): Query<FormatQuery>,
	State(asset): State<service::Asset>,
) -> Result<impl IntoResponse> {
	let format = format_query.format;

	let bytes = asset.convert(version_key, &path, format)?;

	let filename = std::path::Path::new(&path).with_extension(format.extension());
	let disposition = match filename.file_name().and_then(|name| name.to_str()) {
		Some(name) => format!("inline; filename=\"{name}\""),
		None => "inline".to_string(),
	};

	Ok((
		TypedHeader(ContentType::from(format_mime(format))),
		// TypedHeader only has a really naive inline value with no ability to customise :/
		[(header::CONTENT_DISPOSITION, disposition)],
		bytes,
	))
}

fn format_mime(format: Format) -> mime::Mime {
	match format {
		Format::Png => mime::IMAGE_PNG,
	}
}
