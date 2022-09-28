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

	// TODO: this should be shared logic with subrows
	// TODO: schema should be a shared resource in some way so we don't need to check the git repo every request
	// TODO: this would presumably be specified as a provider:version pair in some way
	let schema_provider = ironworks_schema::saint_coinach::Provider::new()?;
	// TODO: as part of said shared resource, need a way to handle updating the repo
	let schema_version = schema_provider.version("HEAD")?;
	let schema_sheet = schema_version.sheet(&sheet_name);

	let result = match schema_sheet {
		Ok(sheet) => read_sheet(&sheet, &row),
		Err(err) => todo!("no schema found because {}, what is the fallback?", err),
	};

	// Ok(format!("{:#?}", result))
	Ok(result)

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

// TODO: this should be elsewhere
// TODO: need some representation of filtering for this, preferably that will be constructable from reference filters, gql queries, and a get request for rest
// TODO: this shouldn't return a string, i don't think. need some arbitrary nested format (nested dicts?) that can be translated depending on what format we're using
fn read_sheet(sheet: &ironworks_schema::Sheet, row: &ironworks::excel::Row) -> String {
	if sheet.order != ironworks_schema::Order::Index {
		todo!("sheet schema {:?} order", sheet.order);
	}

	read_node(0, &sheet.node, row)
}

fn read_node(index: u32, node: &ironworks_schema::Node, row: &ironworks::excel::Row) -> String {
	match node {
		ironworks_schema::Node::Scalar => read_scalar(index, row),
		ironworks_schema::Node::Struct(definition) => read_struct(index, definition, row),
		node => format!("TODO FORMAT {node:?}"),
	}
}

fn read_scalar(index: u32, row: &ironworks::excel::Row) -> String {
	format!("{:?}", row.field(index.try_into().unwrap()))
}

fn read_struct(
	index: u32,
	definition: &[(String, ironworks_schema::Node)],
	row: &ironworks::excel::Row,
) -> String {
	definition
		.iter()
		.scan(index, |index, (key, node)| {
			// TODO: this is wasteful, given it's going to recurse every child node to find the size - is that a problem? probably?
			let result = read_node(*index, node, row);
			*index += node.size();
			Some(format!("{key}: {}\n", result))
		})
		.collect::<String>()
}
