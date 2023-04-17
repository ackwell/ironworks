use askama::Template;
use axum::{
	debug_handler,
	extract::{OriginalUri, Path, State},
	http::Uri,
	response::IntoResponse,
	routing::get,
	Router,
};

use crate::version::{Patch, VersionKey};

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
	OriginalUri(uri): OriginalUri,
	State(version): State<service::Version>,
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
	names: Vec<String>,
	patch_list: Vec<(String, Vec<Patch>)>,
}

#[debug_handler]
async fn version(
	Path(version_key): Path<VersionKey>,
	State(version): State<service::Version>,
) -> Result<impl IntoResponse> {
	// Patches are stored in oldest-first order for IW, which is lovely in code
	// and horrible for reading. Given this is ostensibly the reading bit of the
	// application, fix that.
	let patch_list = version
		.patch_list(version_key)?
		.into_iter()
		.map(|(repository, patches)| (repository, patches.into_iter().rev().collect()))
		.collect();

	Ok(VersionTemplate {
		version: version_key,
		names: version.names(version_key),
		patch_list,
	})
}
