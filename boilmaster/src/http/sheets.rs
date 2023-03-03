use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use ironworks::{excel::Language, file::exh};
use serde::Deserialize;

use crate::{
	data::{Data, LanguageString},
	field_filter::FieldFilter,
	read, schema,
	utility::warnings::Warnings,
};

use super::{
	error::{Anyhow, Error, Result},
	path::Path,
};

pub fn router() -> Router {
	let row_router = Router::new()
		.route("/", get(row))
		.route("/:subrow_id", get(subrow));

	Router::new()
		.route("/", get(sheets))
		.nest("/:sheet_name/:row_id", row_router)
}

#[debug_handler]
async fn sheets(Extension(data): Extension<Arc<Data>>) -> Result<impl IntoResponse> {
	let excel = data.version(None).excel();

	let list = excel.list().anyhow()?;

	// This contains quite a lot of quest/ and custom/ - should I filter them out? Or support them better?
	let names = list.iter().map(|x| x.into_owned()).collect::<Vec<_>>();

	Ok(Json(names))
}

// TODO: this probably should be generally accessible across all sheet endpoints? and search?
#[derive(Deserialize)]
struct FieldFilterQuery {
	// this is a bit jank with the double option, improve? is it even possible to improve?
	fields: Option<Warnings<Option<FieldFilter>>>,
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

#[debug_handler]
async fn row(
	Path((sheet_name, row_id)): Path<(String, u32)>,
	Query(field_filter_query): Query<FieldFilterQuery>,
	Query(schema_query): Query<SchemaQuery>,
	Query(language_query): Query<LanguageQuery>,
	Extension(data): Extension<Arc<Data>>,
	Extension(schema_provider): Extension<Arc<schema::Provider>>,
) -> Result<impl IntoResponse> {
	let excel = data.version(None).excel();

	let sheet = excel.sheet(&sheet_name)?;
	if sheet.kind()? == exh::SheetKind::Subrows {
		return Err(Error::Invalid(format!(
			"Sheet {sheet_name:?} requires a sub-row ID."
		)));
	}

	let schema = schema_provider.schema(schema_query.schema.as_ref())?;

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
		0, // subrow_id,
	)?;

	Ok(Json(result))
}

#[debug_handler]
async fn subrow(
	Path((sheet_name, row_id, subrow_id)): Path<(String, u32, u16)>,
	Query(field_filter_query): Query<FieldFilterQuery>,
	Query(schema_query): Query<SchemaQuery>,
	Query(language_query): Query<LanguageQuery>,
	Extension(data): Extension<Arc<Data>>,
	Extension(schema_provider): Extension<Arc<schema::Provider>>,
) -> Result<impl IntoResponse> {
	let excel = data.version(None).excel();

	let sheet = excel.sheet(&sheet_name)?;
	if sheet.kind()? != exh::SheetKind::Subrows {
		return Err(Error::Invalid(format!(
			"Sheet {sheet_name:?} does not support sub-rows."
		)));
	}

	let schema = schema_provider.schema(schema_query.schema.as_ref())?;

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
		subrow_id,
	)?;

	Ok(Json(result))
}
