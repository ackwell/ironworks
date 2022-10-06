use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;

use crate::{data::Data, search::Search};

use super::error::Result;

pub fn router(search_service: Search) -> Router {
	Router::new()
		.route("/", get(search))
		.layer(Extension(Arc::new(search_service)))
}

#[debug_handler]
async fn search(
	Extension(search): Extension<Arc<Search>>,
	Extension(data): Extension<Arc<Data>>,
) -> Result<impl IntoResponse> {
	let search_version = search.version(None);
	let excel = data.version(None).excel();

	let results = search_version
		.search("summon")?
		.map(|(score, (row_id, subrow_id))| -> Result<_> {
			// TODO: consume index -> sheet tags once those are exposed
			let temp_sheet = excel.sheet("Action")?;
			let row = temp_sheet.subrow(row_id, subrow_id)?;
			// TODO: parse properly
			let name = row.field(0)?.as_string().unwrap().to_string();

			Ok((score, name))
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(Json(results))
}
