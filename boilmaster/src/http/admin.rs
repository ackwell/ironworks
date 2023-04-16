use askama::Template;
use axum::{
	debug_handler,
	extract::{OriginalUri, Path, State},
	http::Uri,
	response::IntoResponse,
	routing::get,
	Router,
};

use crate::version::VersionKey;

use super::{error::Result, service};

pub fn router() -> Router<service::State> {
	Router::new()
		.route("/", get(versions))
		.route("/:version_key", get(version))
}

#[derive(Template)]
#[template(path = "admin/versions.html")]
struct VersionsTemplate {
	// TODO: I imagine the current uri, along with some other stuff, will be really commonly required. Look into how that can be handled.
	current_uri: Uri,
	versions: Vec<VersionInfo>,
}

struct VersionInfo {
	key: VersionKey,
	patches: Vec<(String, String)>,
	names: Vec<String>,
}

#[debug_handler]
async fn versions(
	State(version): State<service::Version>,
	OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse> {
	let version_info = |key: VersionKey| -> Result<_> {
		let latest = version
			.patch_list(key)?
			.into_iter()
			.filter_map(|(repository, patches)| {
				Some((repository, patches.into_iter().last()?.name))
			})
			.collect();

		Ok(VersionInfo {
			key,
			patches: latest,
			names: version.names(key),
		})
	};

	let versions = version
		.versions()
		.into_iter()
		.map(version_info)
		.collect::<Result<Vec<_>>>()?;

	Ok(VersionsTemplate {
		current_uri: uri,
		versions,
	})
}

#[derive(Template)]
#[template(path = "admin/version.html")]
struct VersionTemplate {
	version: VersionKey,
}

#[debug_handler]
async fn version(Path(version_key): Path<VersionKey>) -> Result<impl IntoResponse> {
	Ok(VersionTemplate {
		version: version_key,
	})
}
