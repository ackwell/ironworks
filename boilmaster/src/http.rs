use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use ironworks::{excel::Excel, ffxiv, sqpack::SqPack};
use tower_http::trace::TraceLayer;

#[derive(thiserror::Error, Debug)]
enum Error {
	#[error("Internal server error.")]
	Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match self {
			Self::Other(ref error) => tracing::error!("{error:?}"),
		}

		(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
	}
}

type Result<T, E = Error> = std::result::Result<T, E>;

trait Anyhow<T> {
	fn anyhow(self) -> std::result::Result<T, anyhow::Error>;
}

impl<T, E> Anyhow<T> for std::result::Result<T, E>
where
	E: std::error::Error + Send + Sync + 'static,
{
	fn anyhow(self) -> Result<T, anyhow::Error> {
		self.map_err(anyhow::Error::new)
	}
}

pub fn router() -> Router {
	let sqpack = SqPack::new(ffxiv::FsResource::search().unwrap());
	let sqpack_ref: &'static _ = Box::leak(Box::new(sqpack));
	let excel = Excel::new(ffxiv::SqPackResource::new(sqpack_ref));

	Router::new()
		.route("/sheets", get(sheets))
		.layer(Extension(Arc::new(excel)))
		.layer(TraceLayer::new_for_http())
}

#[debug_handler]
async fn sheets(
	Extension(excel): Extension<Arc<Excel<ffxiv::SqPackResource<'static, ffxiv::FsResource>>>>,
) -> Result<impl IntoResponse> {
	let list = excel.list().anyhow()?;

	// This contains quite a lot of quest/ and custom/ - should I filter them out?
	let names = list.iter().map(|x| x.into_owned()).collect::<Vec<_>>();

	Ok(Json(names))
}
