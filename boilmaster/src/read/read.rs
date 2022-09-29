use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use ironworks::{excel, file::exh};
use ironworks_schema as schema;

use super::value::{Reference, Value};

#[derive(Clone)]
pub struct ReaderContext<'a> {
	pub excel: &'a excel::Excel<'a>,
	pub schema: &'a dyn schema::Schema,

	pub row: &'a excel::Row,
	pub limit: u8,
}

// TODO: need some representation of filtering for this, preferably that will be constructable from reference filters, gql queries, and a get request for rest
pub fn read_sheet(sheet_name: &str, context: ReaderContext) -> Result<Value> {
	let sheet = context.schema.sheet(sheet_name)?;

	if sheet.order != schema::Order::Index {
		todo!("sheet schema {:?} order", sheet.order);
	}

	read_node(0, &sheet.node, context)
}

fn read_node(index: u32, node: &schema::Node, context: ReaderContext) -> Result<Value> {
	use schema::Node as N;
	match node {
		N::Array { count, node } => read_array(index, *count, node, context),
		N::Reference(targets) => read_reference(index, targets, context),
		N::Scalar => read_scalar(index, context),
		N::Struct(definition) => read_struct(index, definition, context),
	}
}

fn read_array(
	index: u32,
	count: u32,
	node: &schema::Node,
	context: ReaderContext,
) -> Result<Value> {
	let size = node.size();
	let vec = (0..count)
		.scan(index, |index, _| {
			let result = read_node(*index, node, context.clone());
			*index += size;
			Some(result)
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(Value::Array(vec))
}

fn read_reference(
	index: u32,
	targets: &[schema::ReferenceTarget],
	context: ReaderContext,
) -> Result<Value> {
	// Coerce the field to a i32
	let field = context.row.field(index.try_into().unwrap())?;
	// TODO: i'd like to include the field in the context but it's really not worth copying the field for.
	let target_value = field_to_index(field).context("failed to convert reference key to i32")?;

	// Build the base reference representation.
	let mut reference = Reference::new(target_value);

	// TODO: is neg case always gonna be like this?
	// A target < 0 (typically -1) signifies that no link is active on this row.
	// Also ensure that we've not run out of recursion space.
	// TODO: should we limit check only just before we run the recursion so the reference data at least includes the target values?
	if target_value < 0 || context.limit == 0 {
		return Ok(Value::Reference(reference));
	}
	let target_value = u32::try_from(target_value).unwrap();

	for target in targets {
		// TODO: condition
		if target.condition.is_some() {
			tracing::warn!("unhandled target condition: {target:?}");
			break;
		}

		// Get the target sheet's data and schema. Intentionally fail hard, as any
		// mismatch here can cause incorrect joins.
		let sheet_data = context.excel.sheet(&target.sheet)?;
		// let sheet_schema = context.schema.sheet(&target.sheet)?;

		// TODO: non-id targets. how will this work alongside subrows?
		if target.selector.is_some() {
			tracing::warn!("unhandled target selector: {target:?}");
			break;
		}

		// TODO: subrows
		if sheet_data.kind()? == exh::SheetKind::Subrows {
			tracing::warn!("unhandled subrow target: {}", target.sheet);
			break;
		}

		// Get the row data for the target. If the row can't be found, pass on to the next target.
		let row_data = match sheet_data.row(target_value) {
			Err(ironworks::Error::NotFound(ironworks::ErrorValue::Row { .. })) => continue,
			other => other,
		}?;

		reference.sheet = Some(target.sheet.clone());
		reference.data = Some(
			read_sheet(
				&target.sheet,
				ReaderContext {
					row: &row_data,
					limit: context.limit - 1,
					..context
				},
			)?
			.into(),
		);
		break;
	}

	Ok(Value::Reference(reference))
}

fn field_to_index(field: excel::Field) -> Result<i32> {
	use excel::Field as F;
	let result = match field {
		F::I8(value) => i32::from(value),
		F::I16(value) => i32::from(value),
		F::I32(value) => value,
		F::I64(value) => value.try_into()?,
		F::U8(value) => i32::from(value),
		F::U16(value) => i32::from(value),
		F::U32(value) => value.try_into()?,
		F::U64(value) => value.try_into()?,

		other => Err(anyhow!("invalid index type {other:?}"))?,
	};
	Ok(result)
}

fn read_scalar(index: u32, context: ReaderContext) -> Result<Value> {
	Ok(Value::Scalar(context.row.field(index.try_into().unwrap())?))
}

fn read_struct(
	index: u32,
	definition: &[(String, schema::Node)],
	context: ReaderContext,
) -> Result<Value> {
	let map = definition
		.iter()
		.scan(index, |index, (key, node)| {
			// TODO: this is wasteful, given it's going to recurse every child node to find the size - is that a problem? probably?
			let result = read_node(*index, node, context.clone());
			*index += node.size();
			Some(result.map(|value| (key.clone(), value)))
		})
		.collect::<Result<HashMap<_, _>>>()?;

	Ok(Value::Struct(map))
}
