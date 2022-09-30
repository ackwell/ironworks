use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use ironworks::{
	excel::{Excel, Row},
	file::exh,
};
use ironworks_schema::saint_coinach;

use crate::{data::Data, read};

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

#[debug_handler]
async fn row(
	Path((sheet_name, row_id)): Path<(String, u32)>,
	Extension(data): Extension<Arc<Data>>,
) -> Result<impl IntoResponse> {
	let excel = data.version(None).excel();

	let sheet = excel.sheet(&sheet_name)?;
	if sheet.kind()? == exh::SheetKind::Subrows {
		return Err(Error::Invalid(format!(
			"Sheet {sheet_name:?} requires a sub-row ID."
		)));
	}

	let row = sheet.row(row_id)?;

	let result = read_row(&sheet_name, excel, &row)?;

	Ok(Json(result))
}

#[debug_handler]
async fn subrow(
	Path((sheet_name, row_id, subrow_id)): Path<(String, u32, u16)>,
	Extension(data): Extension<Arc<Data>>,
) -> Result<impl IntoResponse> {
	let excel = data.version(None).excel();

	let sheet = excel.sheet(&sheet_name)?;
	if sheet.kind()? != exh::SheetKind::Subrows {
		return Err(Error::Invalid(format!(
			"Sheet {sheet_name:?} does not support sub-rows."
		)));
	}

	let row = sheet.subrow(row_id, subrow_id)?;

	let result = read_row(&sheet_name, excel, &row)?;

	Ok(Json(result))
}

fn read_row(sheet_name: &str, excel: &Excel, row: &Row) -> Result<read::Value> {
	// TODO: schema should be a shared resource in some way so we don't need to check the git repo every request
	// TODO: this would presumably be specified as a provider:version pair in some way
	// TODO: as part of said shared resource, need a way to handle updating the repo
	let provider = saint_coinach::Provider::new()?;
	let version = provider.version("HEAD")?;

	let value = read::read_sheet(
		sheet_name,
		read::ReaderContext {
			excel,
			schema: &version,
			row,
			limit: 1,
		},
	)?;

	Ok(value)
}
