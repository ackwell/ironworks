use std::io::Cursor;

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
use ironworks::file::tex;
use itertools::Itertools;
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

	// TODO: obviously this needs a lot of actual proper handling i.e. not assuming it's a 2d rgba texture lmao
	let texture = ironworks.file::<tex::Texture>(&path)?;
	if texture.format() != tex::Format::Argb8 {
		Err(anyhow::anyhow!("unexpected format {:?}", texture.format()))?;
	}

	if texture.dimension() != tex::Dimension::D2 {
		Err(anyhow::anyhow!(
			"unexpected dimension {:?}",
			texture.dimension()
		))?;
	}

	// TODO: seems really wasteful to copy the entire image in memory just to reassign the channels. think of a better way to do this.
	// TODO: consider writing a chunk iterator that uses exact widths rather than moving into a tuple
	let rgba_data = texture
		.data()
		.iter()
		.tuples()
		.flat_map(|(b, g, r, a)| [r, g, b, a])
		.copied()
		.collect::<Vec<_>>();

	let buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
		texture.width().into(),
		texture.height().into(),
		rgba_data,
	)
	.context("texture data smaller than size")?;

	let mut bytes = Cursor::new(vec![]);
	buffer
		.write_to(&mut bytes, image::ImageOutputFormat::Png)
		.context("todo: error handling")?;

	let mut filename = std::path::PathBuf::from(path);
	filename.set_extension("png");
	let disposition = match filename.file_name().and_then(|name| name.to_str()) {
		Some(name) => format!("inline; filename=\"{name}\""),
		None => "inline".to_string(),
	};

	Ok((
		TypedHeader(ContentType::png()),
		// TypedHeader only has a really naive inline value with no ability to customise :/
		[(header::CONTENT_DISPOSITION, disposition)],
		bytes.into_inner(),
	))
}
