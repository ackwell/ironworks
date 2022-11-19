use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::{data::Data, search::Search};

use super::error::Result;

pub fn router(search_service: Arc<Search>) -> Router {
	Router::new()
		.route("/", get(search))
		.layer(Extension(search_service))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
	string: String,
}

#[debug_handler]
async fn search(
	Extension(search): Extension<Arc<Search>>,
	Extension(data): Extension<Arc<Data>>,
	Query(search_query): Query<SearchQuery>,
) -> Result<impl IntoResponse> {
	let search_version = search.version(None);
	let excel = data.version(None).excel();

	let results = search_version
		.search(&search_query.string)?
		.into_iter()
		.map(|(score, (sheet_name, row_id, subrow_id))| -> Result<_> {
			let temp_sheet = excel.sheet(&sheet_name)?;
			let row = temp_sheet.subrow(row_id, subrow_id)?;
			// TODO: parse properly
			let name = row
				.field(0)?
				.as_string()
				.map(|se_string| se_string.to_string());

			Ok((score, sheet_name, name))
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(Json(results))
}
