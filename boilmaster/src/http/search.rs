use std::collections::HashSet;

use anyhow::Context;
use axum::{
	debug_handler,
	extract::{Query, State},
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use ironworks::excel::Language;
use serde::{Deserialize, Serialize};

use crate::{data::LanguageString, schema, search::query};

use super::{error::Result, service};

pub fn router() -> Router<service::State> {
	Router::new().route("/", get(search))
}

#[derive(Deserialize)]
struct VersionQuery {
	version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
	sheets: Option<String>,
	query: query::pre::Node,
}

// TODO: reuse this with sheets
#[derive(Deserialize)]
struct SchemaQuery {
	schema: Option<schema::Specifier>,
}

#[derive(Deserialize)]
struct LanguageQuery {
	language: Option<LanguageString>,
}

// TODO: flesh this out - at the moment it's just a 1:1 of searchresult, pending ideas on how to field filter for search results across multiple indices
#[derive(Debug, Serialize)]
struct SearchResult {
	score: f32,
	sheet: String,
	row_id: u32,
	subrow_id: u16,
}

#[allow(clippy::too_many_arguments)]
#[debug_handler(state = service::State)]
async fn search(
	Query(version_query): Query<VersionQuery>,
	Query(search_query): Query<SearchQuery>,
	Query(schema_query): Query<SchemaQuery>,
	Query(language_query): Query<LanguageQuery>,
	State(data): State<service::Data>,
	State(schema_provider): State<service::Schema>,
	State(search): State<service::Search>,
	State(version): State<service::Version>,
) -> Result<impl IntoResponse> {
	let version_key = version
		.resolve(version_query.version.as_deref())
		.with_context(|| format!("unknown version {:?}", version_query.version))?;
	let search_version = search
		.version(&version_key)
		.context("search index not ready")?;
	let excel = data
		.version(&version_key)
		.context("data not ready")?
		.excel();

	let language = language_query
		.language
		.map(Language::from)
		.unwrap_or_else(|| data.default_language());

	// TODO: I imagine comma-seperated stuff might be relatively common; make a deser helper (probs can trait it up so any fromiter<string> can deser using this pattern)
	let sheets = search_query.sheets.map(|encoded| {
		encoded
			.split(',')
			.map(|x| x.to_owned())
			.collect::<HashSet<_>>()
	});

	let schema = schema_provider.schema(schema_query.schema.as_ref())?;

	let (results, warnings) = search_version
		.search(
			&search_query.query,
			language,
			sheets,
			&excel,
			schema.as_ref(),
		)?
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
