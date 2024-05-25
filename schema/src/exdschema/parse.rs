use std::collections::HashMap;

use serde::Deserialize;

use crate::{
	error::{Error, Result},
	schema,
};

// TODO: unpub this
#[derive(Debug, Deserialize)]
pub struct Sheet {
	name: String,
	display_field: Option<String>,
	fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
	name: String,
	count: Option<u32>,

	#[serde(rename = "type", default)]
	kind: FieldKind,

	comment: Option<String>,
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
	switch: Option<String>,
	cases: Option<HashMap<u32, Vec<String>>>,
}

pub fn parse(data: &[u8]) -> Result<Sheet> {
	let sheet = serde_yaml::from_slice(data)
		.map_err(|error| Error::Schema(format!("failed to parse schema definition: {error}")))?;
	Ok(sheet)
}

