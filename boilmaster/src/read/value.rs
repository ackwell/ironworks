use std::collections::HashMap;

use ironworks::excel;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
	Array(Vec<Value>),
	Reference(Reference),
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

// TODO: finalise this
#[derive(Debug, Serialize)]
pub struct Reference {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sheet: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub key: Option<String>,

	pub value: i32,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Box<Value>>,
}

impl Reference {
	pub fn new(value: i32) -> Self {
		Reference {
			sheet: None,
			key: None,
			value,
			data: None,
		}
	}
}
