use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use ironworks::{excel::Excel, file::exh};

use crate::read;

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

	// This contains quite a lot of quest/ and custom/ - should I filter them out? Or support them better?
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

	// TODO: should any of this schema stuff be in the read module?
	// TODO: this should be shared logic with subrows
	// TODO: schema should be a shared resource in some way so we don't need to check the git repo every request
	// TODO: this would presumably be specified as a provider:version pair in some way
	let schema_provider = ironworks_schema::saint_coinach::Provider::new()?;
	// TODO: as part of said shared resource, need a way to handle updating the repo
	let schema_version = schema_provider.version("HEAD")?;
	let schema_sheet = schema_version.sheet(&sheet_name);

	let result = match schema_sheet {
		Ok(sheet) => read::read_sheet(&sheet, &row)?,
		Err(err) => todo!("no schema found because {}, what is the fallback?", err),
	};

	// Ok(format!("{:#?}", result))
	Ok(Json(result))

	// Ok(format!("{:#?}", row.field(0)))
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
