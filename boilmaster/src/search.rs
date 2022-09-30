use std::{env::current_exe, fs};

use anyhow::{Context, Result};
use ironworks::{
	excel::{Excel, Field, Sheet},
	file::exh,
};
use tantivy::{
	directory::MmapDirectory,
	schema::{self, Schema},
	Document, Index, IndexSettings,
};

pub fn temp_test_search(excel: &Excel) -> Result<()> {
	// TODO: just using action as a test case
	let sheet = excel.sheet("Action")?;

	tracing::info!("building index");

	// TODO: configurable directory, this shouldn't be touching current exe at all
	let path = current_exe()?
		.parent()
		.context("path has no parent")?
		.join("search");
	fs::create_dir_all(&path)?;

	let directory = MmapDirectory::open(path)?;

	// TODO: this should likely be the part of the path that gets joined onto the base or similar
	let index_name = "TODO";
	let index = match Index::exists(&directory)? {
		// NOTE: this assumes that the schema will be effectively immutable
		true => {
			tracing::debug!("using existing index {index_name:?}");
			Index::open(directory)?
		}
		false => {
			tracing::debug!("building {index_name:?}");
			let schema = build_sheet_schema(&sheet)?;
			Index::create(directory, schema, IndexSettings::default())?
		}
	};

	let schema = index.schema();

	// TODO: this can probably just use the ::exists above, and ingestion should be a oneshot anyway.
	let segment_metas = index.searchable_segment_metas();
	tracing::info!("segment metas: {segment_metas:?}");
	let has_docs = segment_metas?.iter().any(|meta| meta.num_docs() > 0);
	tracing::info!("has docs: {has_docs}");

	if has_docs {
		// lmao
		tracing::info!("bailing before ingest");
		return Ok(());
	}

	let mut writer = index.writer(50 * 1024 * 1024)?;

	// test with single row
	// TODO: need an iterator of some kind for this.
	tracing::info!("writing row");
	let row = sheet.row(25831)?;
	let mut document = Document::new();
	for (index, column) in sheet.columns()?.iter().enumerate() {
		let field = schema.get_field(&column.offset().to_string()).unwrap();
		// TODO: this would really value .field(impl intocolumn) or similar
		let value = row.field(index)?;
		// TODO: this feels pretty repetetive given the column kind schema build - is it avoidable or nah?
		use Field as F;
		match value {
			F::String(value) => document.add_text(field, value),

			F::I8(value) => document.add_i64(field, i64::from(value)),
			F::I16(value) => document.add_i64(field, i64::from(value)),
			F::I32(value) => document.add_i64(field, i64::from(value)),
			F::I64(value) => document.add_i64(field, value),

			F::U8(value) => document.add_u64(field, u64::from(value)),
			F::U16(value) => document.add_u64(field, u64::from(value)),
			F::U32(value) => document.add_u64(field, u64::from(value)),
			F::U64(value) => document.add_u64(field, value),

			F::F32(value) => document.add_f64(field, f64::from(value)),

			F::Bool(value) => document.add_u64(field, u64::from(value)),
		}
	}

	// this can block; which suggests to me that at minimum, ingesting should be done on the side.
	writer.add_document(document)?;

	writer.commit()?;

	tracing::info!("opened index {index:?}");

	Ok(())
}

fn build_sheet_schema(sheet: &Sheet<&str>) -> Result<Schema> {
	let mut schema_builder = Schema::builder();

	schema_builder.add_u64_field("id", schema::STORED);
	for column in sheet.columns()? {
		// TODO: technically speaking, using offset means both offset-ordered and column-ordered will work. right?
		let name = column.offset().to_string();

		use exh::ColumnKind as CK;
		match column.kind() {
			// TODO: per-language columns. at the moment, this is just english
			CK::String => schema_builder.add_text_field(&name, schema::TEXT),

			CK::Int8 | CK::Int16 | CK::Int32 | CK::Int64 => {
				schema_builder.add_i64_field(&name, schema::INDEXED)
			}

			CK::UInt8 | CK::UInt16 | CK::UInt32 | CK::UInt64 => {
				schema_builder.add_u64_field(&name, schema::INDEXED)
			}

			CK::Float32 => schema_builder.add_f64_field(&name, schema::INDEXED),

			// TODO: not sure how to handle bools...
			CK::Bool
			| CK::PackedBool0
			| CK::PackedBool1
			| CK::PackedBool2
			| CK::PackedBool3
			| CK::PackedBool4
			| CK::PackedBool5
			| CK::PackedBool6
			| CK::PackedBool7 => schema_builder.add_u64_field(&name, schema::INDEXED),
		};
	}

	Ok(schema_builder.build())
}
