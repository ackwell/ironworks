use std::collections::HashMap;

use ironworks::excel;
use ironworks_schema as schema;

use super::value::Value;

// TODO: need some representation of filtering for this, preferably that will be constructable from reference filters, gql queries, and a get request for rest
pub fn read_sheet(sheet: &schema::Sheet, row: &excel::Row) -> anyhow::Result<Value> {
	if sheet.order != schema::Order::Index {
		todo!("sheet schema {:?} order", sheet.order);
	}

	read_node(0, &sheet.node, row)
}

fn read_node(index: u32, node: &schema::Node, row: &excel::Row) -> anyhow::Result<Value> {
	use schema::Node as N;
	match node {
		N::Array { count, node } => read_array(index, *count, node, row),
		N::Reference(targets) => read_reference(index, targets, row),
		N::Scalar => read_scalar(index, row),
		N::Struct(definition) => read_struct(index, definition, row),
	}
}

fn read_array(
	index: u32,
	count: u32,
	node: &schema::Node,
	row: &excel::Row,
) -> anyhow::Result<Value> {
	let size = node.size();
	let vec = (0..count)
		.scan(index, |index, _| {
			let result = read_node(*index, node, row);
			*index += size;
			Some(result)
		})
		.collect::<anyhow::Result<Vec<_>>>()?;

	Ok(Value::Array(vec))
}

fn read_reference(
	index: u32,
	targets: &[schema::ReferenceTarget],
	row: &excel::Row,
) -> anyhow::Result<Value> {
	tracing::warn!("Unhandled reference type: {targets:?}");
	read_scalar(index, row)
}

fn read_scalar(index: u32, row: &excel::Row) -> anyhow::Result<Value> {
	Ok(Value::Scalar(row.field(index.try_into().unwrap())?))
}

fn read_struct(
	index: u32,
	definition: &[(String, schema::Node)],
	row: &excel::Row,
) -> anyhow::Result<Value> {
	let map = definition
		.iter()
		.scan(index, |index, (key, node)| {
			// TODO: this is wasteful, given it's going to recurse every child node to find the size - is that a problem? probably?
			let result = read_node(*index, node, row);
			*index += node.size();
			Some(result.map(|value| (key.clone(), value)))
		})
		.collect::<anyhow::Result<HashMap<_, _>>>()?;

	Ok(Value::Struct(map))
}
