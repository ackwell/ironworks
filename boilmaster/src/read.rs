use std::collections::HashMap;

use ironworks::excel;
use ironworks_schema as schema;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
	Array(Vec<Value>),
	// TODO: should references even exist in this enum, or should we eagerly resolve them? I'm tempted to say the latter.
	#[serde(serialize_with = "serialize_scalar")]
	Scalar(excel::Field),
	Struct(HashMap<String, Value>),
}

// TODO: this is effectively just making up for serialize not being impl'd in iw::excel - should that be enabled under a feature or is it better to do over here as we are?
fn serialize_scalar<S: Serializer>(field: &excel::Field, s: S) -> Result<S::Ok, S::Error> {
	use excel::Field as F;

	match field {
		// TODO: more comprehensive sestring handling
		F::String(se_string) => s.serialize_str(&se_string.to_string()),
		F::Bool(value) => s.serialize_bool(*value),
		F::I8(value) => s.serialize_i8(*value),
		F::I16(value) => s.serialize_i16(*value),
		F::I32(value) => s.serialize_i32(*value),
		F::I64(value) => s.serialize_i64(*value),
		F::U8(value) => s.serialize_u8(*value),
		F::U16(value) => s.serialize_u16(*value),
		F::U32(value) => s.serialize_u32(*value),
		F::U64(value) => s.serialize_u64(*value),
		F::F32(value) => s.serialize_f32(*value),
	}
}

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
