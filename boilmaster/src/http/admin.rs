use axum::{
	debug_handler,
	extract::{OriginalUri, Path, State},
	response::IntoResponse,
	routing::get,
	Router,
};
use humansize::{format_size, BINARY};
use maud::{html, Markup, Render, DOCTYPE};

use crate::version::{Patch, VersionKey};

use super::{error::Result, service};

pub fn router() -> Router<service::State> {
	Router::new()
		.route("/", get(versions))
		.route("/:version_key", get(version))
}

struct BaseTemplate {
	title: String,
	content: Markup,
}

impl Render for BaseTemplate {
	fn render(&self) -> Markup {
		html! {
			(DOCTYPE)
			html {
				head {
					title { "admin | " (self.title) }
				}
				body {
					h1 { (self.title) }
					(self.content)
				}
			}
		}
	}
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

	Ok((BaseTemplate {
		title: "versions".to_string(),
		content: html! {
			@for version in versions {
				h2 {
					a href={ (uri) "/" (version.key) } {
						(version.key)
					}

					" ("
					@for (index, name) in version.names.iter().enumerate() {
						@if index > 0 { ", " }
						(name)
					}
					")"
				}

				dl {
					@for (repository, patch) in &version.patches {
						dt { (repository) }
						dd { (patch) }
					}
				}
			}
		},
	})
	.render())
}

#[debug_handler]
async fn version(
	Path(version_key): Path<VersionKey>,
	State(version): State<service::Version>,
) -> Result<impl IntoResponse> {
	let names = version.names(version_key);

	// Patches are stored in oldest-first order for IW, which is lovely in code
	// and horrible for reading. Given this is ostensibly the reading bit of the
	// application, fix that.
	let patch_list = version
		.patch_list(version_key)?
		.into_iter()
		.map(|(repository, patches)| (repository, patches.into_iter().rev().collect()))
		.collect::<Vec<(String, Vec<Patch>)>>();

	Ok((BaseTemplate {
		title: format!("version {}", version_key),
		content: html! {
			h2 { "names" }
			ul {
				@for name in names {
					li { (name) }
				}
			}

			h2 { "patches" }
			@for (repository, patches) in patch_list {
				details {
					summary {
						(repository)
						" ("
						(patches.len()) " patches, "
						"latest: " (patches.first().map(|patch| patch.name.as_str()).unwrap_or("none"))
						")"
					}
					ul {
						@for patch in patches {
							li { (patch.name) " (" (format_size(patch.size, BINARY)) ")" }
						}
					}
				}
			}
		},
	})
	.render())
}
