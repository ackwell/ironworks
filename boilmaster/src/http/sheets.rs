use anyhow::Context;
use axum::{
	debug_handler,
	extract::{Query, State},
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use ironworks::{excel::Language, file::exh};
use serde::Deserialize;

use crate::{
	data::LanguageString,
	read, schema,
	utility::{anyhow::Anyhow, warnings::Warnings},
};

use super::{
	error::{Error, Result},
	path::Path,
	service,
};

pub fn router() -> Router<service::State> {
	Router::new()
		.route("/", get(sheets))
		.route("/:sheet_name/:row_id", get(row))
		.route("/:sheet_name/:row_id/:subrow_id", get(row))
}

#[derive(Debug, Deserialize)]
struct RowPath {
	sheet_name: String,
	row_id: u32,
	subrow_id: Option<u16>,
}

#[debug_handler(state = service::State)]
async fn sheets(
	State(data): State<service::Data>,
	State(version): State<service::Version>,
) -> Result<impl IntoResponse> {
	// TODO: these should be falling back to a default version exposed by version::. also needs a better error
	let version_key = version
		.resolve(None)
		.context("latest should always exist")?;
	let excel = data
		.version(&version_key)
		.context("data not ready")?
		.excel();

	let list = excel.list().anyhow()?;

	// This contains quite a lot of quest/ and custom/ - should I filter them out? Or support them better?
	let names = list.iter().map(|x| x.into_owned()).collect::<Vec<_>>();

	Ok(Json(names))
}

// TODO: this probably should be generally accessible across all sheet endpoints? and search?
#[derive(Deserialize)]
struct FieldFilterQuery {
	// this is a bit jank with the double option, improve? is it even possible to improve?
	fields: Option<Warnings<Option<read::Filter>>>,
}

// TODO: likewise with field filter, should be reuseable
#[derive(Deserialize)]
struct SchemaQuery {
	schema: Option<schema::Specifier>,
}

// TODO: ditto. given these 3 all feed into the concept of reading, perhaps they should be grouped? search will likely accept the same. in saying that, search also uses the schema for searching so maybe not quite that simple
#[derive(Deserialize)]
struct LanguageQuery {
	language: Option<LanguageString>,
}

#[debug_handler(state = service::State)]
async fn row(
	Path(RowPath {
		sheet_name,
		row_id,
		subrow_id,
	}): Path<RowPath>,
	Query(field_filter_query): Query<FieldFilterQuery>,
	Query(schema_query): Query<SchemaQuery>,
	Query(language_query): Query<LanguageQuery>,
	State(data): State<service::Data>,
	State(schema_provider): State<service::Schema>,
	State(version): State<service::Version>,
) -> Result<impl IntoResponse> {
	// TODO: these should be falling back to a default version exposed by version::. also needs a better error
	let version_key = version
		.resolve(None)
		.context("latest should always exist")?;
	let excel = data
		.version(&version_key)
		.context("data not ready")?
		.excel();
	let schema = schema_provider.schema(schema_query.schema.as_ref())?;

	// Sanity check that the correct path was used.
	let sheet_kind = excel.sheet(&sheet_name)?.kind()?;
	if let Some(message) = match (sheet_kind, subrow_id) {
		(exh::SheetKind::Default, Some(_)) => {
			Some(format!("sheet {sheet_name:?} does not support sub-rows"))
		}
		(exh::SheetKind::Subrows, None) => {
			Some(format!("sheet {sheet_name:?} requires a sub-row ID"))
		}
		(exh::SheetKind::Unknown, _) => {
			Some(format!("sheet {sheet_name:?} cannot be read at this time"))
		}
		_ => None,
	} {
		return Err(Error::Invalid(message));
	};

	let (field_filter, warnings) = field_filter_query
		.fields
		.unwrap_or_else(|| Warnings::new(None))
		.decompose();
	if !warnings.is_empty() {
		todo!("handle warnings in http layer");
	}

	let language = language_query
		.language
		.map(Language::from)
		.unwrap_or_else(|| data.default_language());

	let result = read::read(
		&excel,
		schema.as_ref(),
		language,
		field_filter.as_ref(),
		&sheet_name,
		row_id,
		subrow_id.unwrap_or(0),
	)?;

	Ok(Json(result))
}
