use std::sync::Arc;

use axum::{
	extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use axum_macros::debug_handler;
use ironworks::{excel::Excel, ffxiv, sqpack::SqPack, Ironworks};
use tower_http::trace::TraceLayer;

// TODO Should probably be an "api error"?
#[derive(thiserror::Error, Debug)]
enum Error {
	#[error("Not found: {0}")]
	NotFound(String),

	#[error("Internal server error.")]
	Other(#[from] anyhow::Error),
}

impl From<ironworks::Error> for Error {
	fn from(error: ironworks::Error) -> Self {
		use ironworks::Error as IE;
		match error {
			IE::NotFound(value) => Self::NotFound(value.to_string()),
			error => Self::Other(error.into()),
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		// Log the full error for ISEs - we don't show this info anywhere else in case it contains something sensitive.
		if let Self::Other(ref error) = self {
			tracing::error!("{error:?}")
		}

		// TODO: INCREDIBLY IMPORTANT: work out how to worm IM_A_TEAPOT into this
		let status_code = match self {
			Self::NotFound(_) => StatusCode::NOT_FOUND,
			Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
		};

		// TODO: json error response
		(status_code, self.to_string()).into_response()
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
	// TODO: Work out how to handle languages
	let ironworks = Ironworks::new().resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
	let excel = Excel::with()
		.language(ffxiv::Language::English)
		.build(Arc::new(ironworks), ffxiv::Mapper::new());

	Router::new()
		.nest("/sheets", sheets_router())
		.layer(Extension(Arc::new(excel)))
		.layer(TraceLayer::new_for_http())
}

fn sheets_router() -> Router {
	let row_router = Router::new()
		.route("/", get(row))
		.route("/:subrow_id", get(subrow));

	Router::new()
		.route("/", get(sheets))
		.nest("/:sheet_name/:row_id", row_router)
}

#[debug_handler]
async fn sheets(Extension(excel): Extension<Arc<Excel<'static>>>) -> Result<impl IntoResponse> {
	let list = excel.list().anyhow()?;

	// This contains quite a lot of quest/ and custom/ - should I filter them out?
	let names = list.iter().map(|x| x.into_owned()).collect::<Vec<_>>();

	Ok(Json(names))
}

#[debug_handler]
async fn row(
	Path((sheet_name, row_id)): Path<(String, u32)>,
	excel: Extension<Arc<Excel<'static>>>,
) -> Result<impl IntoResponse> {
	// TODO: check sheet kind
	let row = excel.sheet(sheet_name)?.row(row_id)?;

	Ok(format!("{:#?}", row.field(0)))
}

#[debug_handler]
async fn subrow(
	Path((sheet_name, row_id, subrow_id)): Path<(String, u32, u16)>,
	Extension(excel): Extension<Arc<Excel<'static>>>,
) -> Result<impl IntoResponse> {
	// TODO: check sheet kind
	let row = excel.sheet(sheet_name)?.subrow(row_id, subrow_id)?;

	Ok(format!("{:#?}", row.field(0)))
}
