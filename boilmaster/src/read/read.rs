use std::collections::{btree_map::Entry, BTreeMap};

use anyhow::{anyhow, Context, Result};
use ironworks::{excel, file::exh};
use ironworks_schema as schema;

use crate::field_filter::FieldFilter;

use super::value::{Reference, Value};

// Characters to strip from schema struct keys
// TODO: this is potentially a bit saint-specific; but i'm very hesitant to put this logic in stc parsing, as that's technically "wrong". probably best shot is to keep this logic in tune with what BM requires as an output data format.
const FIELD_STRIP_CHARACTERS: &[char] = &['{', '}', '[', ']', '<', '>'];

#[derive(Clone)]
pub struct ReaderContext<'a> {
	pub excel: &'a excel::Excel<'a>,
	pub schema: &'a dyn schema::Schema,
	pub filter: Option<&'a FieldFilter>,

	pub row: &'a excel::Row,
	pub limit: u8,

	pub columns: &'a [exh::ColumnDefinition],
}

pub fn read_sheet(sheet_name: &str, context: ReaderContext) -> Result<Value> {
	let sheet = context.schema.sheet(sheet_name)?;

	if sheet.order != schema::Order::Index {
		todo!("sheet schema {:?} order", sheet.order);
	}

	read_node(&sheet.node, context)
}

fn read_node(node: &schema::Node, context: ReaderContext) -> Result<Value> {
	use schema::Node as N;
	match node {
		N::Array { count, node } => read_array(*count, node, context),
		N::Reference(targets) => read_reference(targets, context),
		N::Scalar => read_scalar(context),
		N::Struct(definition) => read_struct(definition, context),
	}
}

fn read_array(count: u32, node: &schema::Node, context: ReaderContext) -> Result<Value> {
	let inner_filter = match context.filter {
		Some(FieldFilter::Array(inner)) => inner.as_ref().map(|x| x.as_ref()),
		// TODO: should this be a warning?
		Some(other) => return Err(anyhow!("unexpected filter {other}")),
		None => None,
	};

	let size = node.size();
	let vec = (0..count)
		.scan(0usize, |index, _| {
			let size_usize = usize::try_from(size).unwrap();
			let result = read_node(
				node,
				ReaderContext {
					columns: context
						.columns
						.get(*index..*index + size_usize)
						.unwrap_or(&[]),
					filter: inner_filter,
					..context
				},
			);
			*index += size_usize;
			Some(result)
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(Value::Array(vec))
}

fn read_reference(targets: &[schema::ReferenceTarget], context: ReaderContext) -> Result<Value> {
	let column = context.columns.get(0).context("schema mismatch")?;

	// Coerce the field to a i32
	let field = context.row.field(column)?;
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
					columns: &sheet_data.columns()?,
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

fn read_scalar(context: ReaderContext) -> Result<Value> {
	// TODO: schema mismatches are gonna happen - probably should try to fail more gracefully than a 500.
	let column = context.columns.get(0).context("schema mismatch")?;
	Ok(Value::Scalar(context.row.field(column)?))
}

fn read_struct(fields: &[schema::StructField], context: ReaderContext) -> Result<Value> {
	let mut map = BTreeMap::new();

	let filter = match context.filter {
		Some(FieldFilter::Struct(map)) => Some(map),
		// TODO: should this be a warning?
		Some(other) => return Err(anyhow!("unexpected filter {other}")),
		None => None,
	};

	let mut offset = 0usize;
	while offset < context.columns.len() {
		// TODO: this is yikes. Probably can improve with a .next-based thing given fields are ordered
		let field = fields
			.iter()
			.find(|&field| field.offset == u32::try_from(offset).unwrap());

		let (name, size, read): (_, _, Box<dyn FnOnce(_) -> _>) = match field {
			Some(field) => {
				let size = usize::try_from(field.node.size()).unwrap();
				let name = field.name.replace(FIELD_STRIP_CHARACTERS, "");
				let read = |context| read_node(&field.node, context);
				(name, size, Box::new(read))
			}

			None => (format!("unknown{offset}"), 1, Box::new(read_scalar)),
		};

		let range = offset..offset + size;
		offset += size;

		let child_filter = match filter {
			// No filter is present, select all.
			None => None,
			Some(map) => match map.get(&name) {
				// A filter exists, grab the child filter to pass down.
				Some(inner_filter) => inner_filter.as_ref(),
				// The filter doesn't contain this key, skip it.
				None => {
					continue;
				}
			},
		};

		let value = read(ReaderContext {
			columns: context.columns.get(range).unwrap_or(&[]),
			// TODO: filter
			filter: child_filter,
			..context
		})?;

		match map.entry(name) {
			Entry::Vacant(entry) => {
				entry.insert(value);
			}

			Entry::Occupied(entry) => {
				tracing::warn!(name = %entry.key(), "name collision");
			}
		};
	}

	Ok(Value::Struct(map))
}
