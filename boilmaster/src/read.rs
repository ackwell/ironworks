use std::collections::HashMap;

use ironworks::excel;
use ironworks_schema as schema;

#[derive(Debug)]
pub enum Value {
	Array(Vec<Value>),
	// TODO: should references even exist in this enum, or should we eagerly resolve them? I'm tempted to say the latter.
	Scalar(excel::Field),
	Struct(HashMap<String, Value>),
}

// TODO: need some representation of filtering for this, preferably that will be constructable from reference filters, gql queries, and a get request for rest
// TODO: this shouldn't return a string, i don't think. need some arbitrary nested format (nested dicts?) that can be translated depending on what format we're using
pub fn read_sheet(sheet: &schema::Sheet, row: &excel::Row) -> String {
	if sheet.order != schema::Order::Index {
		todo!("sheet schema {:?} order", sheet.order);
	}

	let result = read_node(0, &sheet.node, row);
	format!("{result:#?}")
}

fn read_node(index: u32, node: &schema::Node, row: &excel::Row) -> anyhow::Result<Value> {
	match node {
		schema::Node::Array { count, node } => read_array(index, *count, node, row),
		schema::Node::Scalar => read_scalar(index, row),
		schema::Node::Struct(definition) => read_struct(index, definition, row),
		node => {
			tracing::warn!("Unhandled node type: {node:?}");
			Ok(Value::Scalar(excel::Field::F32(f32::MIN)))
		}
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
