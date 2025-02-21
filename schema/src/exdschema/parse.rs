use std::collections::HashMap;

use serde::Deserialize;

use crate::{
	error::{Error, Result},
	schema,
};

#[derive(Debug, Deserialize)]
struct Sheet {
	name: String,
	_display_field: Option<String>,
	fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
	// Only optional to support single-element arrays.
	name: Option<String>,
	_comment: Option<String>,

	#[serde(rename = "type", default)]
	kind: FieldKind,

	count: Option<u32>,
	fields: Option<Vec<Field>>,

	condition: Option<Condition>,
	targets: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum FieldKind {
	Scalar,
	Array,
	Icon,
	ModelId,
	Color,
	Link,
}

impl Default for FieldKind {
	fn default() -> Self {
		Self::Scalar
	}
}

#[derive(Debug, Deserialize)]
struct Condition {
	switch: String,
	cases: HashMap<u32, Vec<String>>,
}

pub fn parse(data: &[u8]) -> Result<schema::Sheet> {
	let sheet = serde_yaml::from_slice(data)
		.map_err(|error| Error::Schema(format!("failed to parse schema definition: {error}")))?;

	map_sheet(sheet)
}

fn map_sheet(sheet: Sheet) -> Result<schema::Sheet> {
	Ok(schema::Sheet {
		name: sheet.name,
		order: schema::Order::Offset,
		node: map_struct(sheet.fields)?,
	})
}

macro_rules! scan_try {
	($expr:expr) => {
		match $expr {
			Ok(value) => value,
			Err(err) => return Some(Err(err)),
		}
	};
}

fn map_struct(fields: Vec<Field>) -> Result<schema::Node> {
	let struct_fields = fields
		.into_iter()
		.scan(0u32, |offset, field| {
			let name = scan_try!(
				field
					.name
					.clone()
					.ok_or_else(|| Error::Schema(format!("struct fields must have names")))
			);
			let node = scan_try!(map_field(field));

			let this_offset = *offset;
			*offset += node.size();

			Some(Ok(schema::StructField {
				offset: this_offset,
				// TODO: would be good to avoid cloning this but i don't really want to work out partial borrowing for everything.
				name,
				node,
			}))
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(schema::Node::Struct(struct_fields))
}

fn map_field(field: Field) -> Result<schema::Node> {
	// TODO: comment? probably irrelevant for programattic consumption, but would be useful to wire through to i.e. generator. consider - probably a struct level concern, though.

	let node = match field {
		// Scalar columns.
		Field {
			kind: FieldKind::Scalar,
			..
		} => schema::Node::Scalar(schema::Scalar::Default),

		Field {
			kind: FieldKind::Icon,
			..
		} => schema::Node::Scalar(schema::Scalar::Icon),

		Field {
			kind: FieldKind::ModelId,
			..
		} => schema::Node::Scalar(schema::Scalar::Model),

		Field {
			kind: FieldKind::Color,
			..
		} => schema::Node::Scalar(schema::Scalar::Color),

		// Arrays.
		Field {
			kind: FieldKind::Array,
			count: Some(count),
			fields,
			..
		} => {
			let node = match fields {
				None => schema::Node::Scalar(schema::Scalar::Default),
				Some(mut fields) => match fields.len() {
					0 => Err(Error::Schema(format!(
						"arrays must contain at least one field"
					)))?,
					1 => map_field(fields.remove(0))?,
					_ => map_struct(fields)?,
				},
			};

			schema::Node::Array {
				count,
				node: node.into(),
			}
		}

		// Unconditional links.
		Field {
			kind: FieldKind::Link,
			targets: Some(targets),
			condition: None,
			..
		} => schema::Node::Scalar(schema::Scalar::Reference(
			targets
				.into_iter()
				.map(|target| schema::ReferenceTarget {
					sheet: target,
					selector: None,
					condition: None,
				})
				.collect(),
		)),

		// Conditional links.
		Field {
			kind: FieldKind::Link,
			condition: Some(condition),
			targets: None,
			..
		} => {
			let targets = condition
				.cases
				.into_iter()
				.flat_map(|(value, sheets)| sheets.into_iter().map(move |sheet| (sheet, value)))
				.map(|(sheet, value)| schema::ReferenceTarget {
					sheet,
					selector: None,
					condition: Some(schema::ReferenceCondition {
						selector: condition.switch.clone(),
						value,
					}),
				})
				.collect::<Vec<_>>();

			schema::Node::Scalar(schema::Scalar::Reference(targets))
		}

		other => Err(Error::Schema(format!(
			"invalid EXDSchema field declaration: {other:?}"
		)))?,
	};

	Ok(node)
}
