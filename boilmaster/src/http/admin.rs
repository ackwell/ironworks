use askama::Template;
use axum::{debug_handler, extract::State, response::IntoResponse, routing::get, Router};

use crate::version::VersionKey;

use super::{error::Result, service};

pub fn router() -> Router<service::State> {
	Router::new().route("/", get(versions))
}

#[derive(Template)]
#[template(path = "admin/versions.html")]
struct VersionsTemplate {
	versions: Vec<VersionInfo>,
}

struct VersionInfo {
	key: VersionKey,
	patches: Vec<(String, String)>,
	names: Vec<String>,
}

#[debug_handler]
async fn versions(State(version): State<service::Version>) -> Result<impl IntoResponse> {
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

	Ok(VersionsTemplate { versions })
}
