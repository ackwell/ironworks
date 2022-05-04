use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use ironworks::{excel::Excel, file::exh};

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
async fn sheets(Extension(excel): Extension<Arc<Excel<'static>>>) -> Result<impl IntoResponse> {
	let list = excel.list().anyhow()?;

	// This contains quite a lot of quest/ and custom/ - should I filter them out?
	let names = list.iter().map(|x| x.into_owned()).collect::<Vec<_>>();

	Ok(Json(names))
}

#[debug_handler]
async fn row(
	Path((sheet_name, row_id)): Path<(String, u32)>,
	Extension(excel): Extension<Arc<Excel<'static>>>,
) -> Result<impl IntoResponse> {
	let sheet = excel.sheet(&sheet_name)?;
	if sheet.kind()? == exh::SheetKind::Subrows {
		return Err(Error::Invalid(format!(
			"Sheet {sheet_name:?} requires a sub-row ID."
		)));
	}

	let row = sheet.row(row_id)?;

	Ok(format!("{:#?}", row.field(0)))
}

#[debug_handler]
async fn subrow(
	Path((sheet_name, row_id, subrow_id)): Path<(String, u32, u16)>,
	Extension(excel): Extension<Arc<Excel<'static>>>,
) -> Result<impl IntoResponse> {
	let sheet = excel.sheet(&sheet_name)?;
	if sheet.kind()? != exh::SheetKind::Subrows {
		return Err(Error::Invalid(format!(
			"Sheet {sheet_name:?} does not support sub-rows."
		)));
	}

	let row = sheet.subrow(row_id, subrow_id)?;

	Ok(format!("{:#?}", row.field(0)))
}
