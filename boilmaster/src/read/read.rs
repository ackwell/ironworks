use std::{
	cell::RefCell,
	collections::{btree_map, hash_map, BTreeMap, HashMap},
	rc::Rc,
};

use anyhow::{anyhow, Context, Result};
use ironworks::{excel, file::exh};
use ironworks_schema as schema;

use crate::utility::field;

use super::{
	filter::{Filter, StructKey},
	value::{Reference, Value},
};

pub fn read(
	excel: &excel::Excel,
	schema: &dyn schema::Schema,

	language: excel::Language,
	filter: Option<&Filter>,

	sheet_name: &str,
	row_id: u32,
	subrow_id: u16,
) -> Result<Value> {
	let sheet_schema = schema.sheet(sheet_name)?;

	let sheet = excel.sheet(sheet_name)?;
	let columns = sheet.columns()?;

	read_sheet(
		sheet_schema,
		ReaderContext {
			excel,
			schema,

			language,
			row_id,
			subrow_id,

			filter,

			sheet: &sheet,
			rows: Default::default(),
			columns: &columns,
			limit: 1,
		},
	)
}

#[derive(Clone)]
struct ReaderContext<'a> {
	excel: &'a excel::Excel<'a>,
	schema: &'a dyn schema::Schema,

	language: excel::Language,
	row_id: u32,
	subrow_id: u16,

	filter: Option<&'a Filter>,

	sheet: &'a excel::Sheet<'a, &'a str>,
	rows: Rc<RefCell<HashMap<excel::Language, excel::Row>>>,
	columns: &'a [exh::ColumnDefinition],
	limit: u8,
}

impl ReaderContext<'_> {
	fn next_field(&self) -> Result<excel::Field> {
		// Grab the row from cache, creating if not yet retrieved.
		let mut map = self.rows.borrow_mut();
		let row = match map.entry(self.language) {
			hash_map::Entry::Occupied(entry) => entry.into_mut(),
			hash_map::Entry::Vacant(entry) => entry.insert(
				self.sheet
					.with()
					.language(self.language)
					.subrow(self.row_id, self.subrow_id)?,
			),
		};

		// TODO: schema mismatches are gonna happen - probably should try to fail more gracefully than a 500.
		let column = self.columns.get(0).context("schema mismatch")?;

		Ok(row.field(column)?)
	}
}

fn read_sheet(sheet: schema::Sheet, context: ReaderContext) -> Result<Value> {
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
		Some(Filter::Array(inner)) => inner.as_ref().map(|x| x.as_ref()),
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

					rows: context.rows.clone(),
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
	// Coerce the field to a i32
	let field = context.next_field()?;
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
		let sheet_data = context.excel.sheet(target.sheet.as_str())?;
		let sheet_schema = context.schema.sheet(&target.sheet)?;

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
		let row_data = match sheet_data
			.with()
			.language(context.language)
			.row(target_value)
		{
			Err(ironworks::Error::NotFound(ironworks::ErrorValue::Row { .. })) => continue,
			other => other,
		}?;

		reference.sheet = Some(target.sheet.clone());
		reference.data = Some(
			read_sheet(
				sheet_schema,
				ReaderContext {
					row_id: row_data.row_id(),
					subrow_id: row_data.subrow_id(),
					sheet: &sheet_data,
					rows: Rc::new(RefCell::new(HashMap::from([(context.language, row_data)]))),
					columns: &sheet_data.columns()?,
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

fn read_scalar(context: ReaderContext) -> Result<Value> {
	// TODO: schema mismatches are gonna happen - probably should try to fail more gracefully than a 500.
	Ok(Value::Scalar(context.next_field()?))
}

fn read_struct(fields: &[schema::StructField], context: ReaderContext) -> Result<Value> {
	let mut map = BTreeMap::new();

	let filter = match context.filter {
		Some(Filter::Struct(map)) => Some(map),
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

		let (name, size, read): (_, _, Box<dyn Fn(_) -> _>) = match field {
			Some(field) => {
				let size = usize::try_from(field.node.size()).unwrap();
				let name = field::sanitize_name(&field.name);
				let read = |context| read_node(&field.node, context);
				(name, size, Box::new(read))
			}

			None => (format!("unknown{offset}"), 1, Box::new(read_scalar)),
		};

		let range = offset..offset + size;
		offset += size;

		// Get any language-filter pairs for this field, defaulting to a
		// default-language no-op-filter if no filter exists.
		// TODO: Improve this logic, this entire struct reader can probably do with a bit of cleaning up.
		let default_key = StructKey {
			name,
			language: None,
		};
		let inner_filters = match filter {
			None => vec![(&default_key, None)],
			Some(map) => map
				.iter()
				.filter_map(|(key, inner_filter)| match key.name == default_key.name {
					true => Some((key, inner_filter.as_ref())),
					false => None,
				})
				.collect::<Vec<_>>(),
		};

		for (key, inner_filter) in inner_filters {
			let value = read(ReaderContext {
				columns: context.columns.get(range.clone()).unwrap_or(&[]),
				language: key
					.language
					.map(excel::Language::from)
					.unwrap_or(context.language),
				filter: inner_filter,
				rows: context.rows.clone(),
				..context
			})?;

			match map.entry(key.to_string()) {
				btree_map::Entry::Vacant(entry) => {
					entry.insert(value);
				}

				btree_map::Entry::Occupied(entry) => {
					tracing::warn!(name = %entry.key(), "name collision");
				}
			};
		}
	}

	Ok(Value::Struct(map))
}
