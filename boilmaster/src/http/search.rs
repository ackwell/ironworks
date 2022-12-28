use std::{collections::HashSet, sync::Arc};

use anyhow::Context;
use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
	data::Data,
	schema,
	search::{query, Search},
};

use super::error::Result;

pub fn router(search_service: Arc<Search>) -> Router {
	Router::new()
		.route("/", get(search))
		.layer(Extension(search_service))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
	sheets: Option<String>,
	query: query::pre::Node,
}

// TODO: flesh this out - at the moment it's just a 1:1 of searchresult, pending ideas on how to field filter for search results across multiple indices
#[derive(Debug, Serialize)]
struct SearchResult {
	score: f32,
	sheet: String,
	row_id: u32,
	subrow_id: u16,
}

#[debug_handler]
async fn search(
	Query(search_query): Query<SearchQuery>,
	Extension(data): Extension<Arc<Data>>,
	Extension(schema_provider): Extension<Arc<schema::Provider>>,
	Extension(search): Extension<Arc<Search>>,
) -> Result<impl IntoResponse> {
	// TODO: this should expose a more useful error to the end user.
	let search_version = search.version(None).context("search index not ready")?;
	let excel = data.version(None).excel();

	// TODO: I imagine comma-seperated stuff might be relatively common; make a deser helper (probs can trait it up so any fromiter<string> can deser using this pattern)
	let sheets = search_query.sheets.map(|encoded| {
		encoded
			.split(',')
			.map(|x| x.to_owned())
			.collect::<HashSet<_>>()
	});

	// TODO: this would presumably be specified as a provider:version pair in some way
	let schema = schema_provider.schema("saint-coinach", None)?;

	let (results, warnings) = search_version
		.search(&search_query.query, sheets, &excel, schema.as_ref())?
		.decompose();

	let http_results = results
		.into_iter()
		.map(|result| SearchResult {
			score: result.score,
			sheet: result.sheet,
			row_id: result.row_id,
			subrow_id: result.subrow_id,
		})
		.collect::<Vec<_>>();

	Ok(Json((http_results, warnings)))
}
