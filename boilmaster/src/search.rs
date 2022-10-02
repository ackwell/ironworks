use std::{env::current_exe, fs};

use anyhow::{Context, Result};
use ironworks::{
	excel::{Excel, Field, Sheet},
	file::exh,
};
use tantivy::{
	collector::TopDocs,
	directory::MmapDirectory,
	query::QueryParser,
	schema::{self, FieldType, Schema},
	Document, Index, IndexSettings, ReloadPolicy,
};

pub fn temp_test_search(excel: &Excel) -> Result<()> {
	// TODO: this should be a more active setup in future
	let version = excel.version()?;
	let sheet_name = "Action";
	let index_name = format!("{version}/{sheet_name}");

	// TODO: just using action as a test case
	// TODO: can probably avoid fetching the sheet if not building
	let sheet = excel.sheet(sheet_name)?;

	// TODO: configurable directory, this shouldn't be touching current exe at all
	let path = current_exe()?
		.parent()
		.context("path has no parent")?
		.join("search")
		.join(version)
		.join(sheet_name);
	fs::create_dir_all(&path)?;

	let directory = MmapDirectory::open(path)?;

	// TODO: this should likely be the part of the path that gets joined onto the base or similar
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
		// lmao - test quering just... to see. to see if it works. i mean why wouldn't it but fuck me i want to spike this without refactoring the trash first. yay trash
		let reader = index
			.reader_builder()
			// TODO: this is set to manual 'cus technically an index is never updated in this setup. is that sane? i dunno?
			.reload_policy(ReloadPolicy::Manual)
			.try_into()?;

		let searcher = reader.searcher();

		// so immediate complication to deal with; need to specify the fields to search if the user (lmao as if) doesn't specify any. we... techncially want to search _every_thing. or, at least every string thing? idfk. all strings makes sense i guess?
		let fields = schema
			.fields()
			.filter(|(_field, entry)| matches!(entry.field_type(), FieldType::Str(_)))
			.map(|(field, _entry)| field)
			.collect::<Vec<_>>();

		tracing::info!("tryna query");
		let query_parser = QueryParser::for_index(&index, fields);
		let query = query_parser.parse_query("summon")?;

		let top_docs = searcher.search(&query, &TopDocs::with_limit(100))?;

		for (score, doc_addresss) in top_docs {
			let doc = searcher.doc(doc_addresss)?;
			let id = doc
				.get_first(schema.get_field("row_id").unwrap())
				.unwrap()
				.as_u64()
				.unwrap();
			let row = sheet.row(id.try_into().unwrap()).unwrap();
			tracing::debug!(
				"{score}: {:?}",
				row.field(0).unwrap().as_string().unwrap().to_string()
			);
		}

		tracing::info!("bailing before ingest");
		return Ok(());
	}

	let mut writer = index.writer(50 * 1024 * 1024)?;

	// TODO: this is nightmare fuel, break up. a lot.
	// TODO: if there's any failures at all (i.e. iw read errors) during ingestion, the writer should be rolled back to ensure a theoretical retry is able to work on a clean deck.
	tracing::debug!("ingesting {index_name}");
	for row in sheet.iter() {
		let mut document = Document::new();

		document.add_u64(schema.get_field("row_id").unwrap(), (*row.row_id()).into());
		document.add_u64(
			schema.get_field("subrow_id").unwrap(),
			(*row.subrow_id()).into(),
		);

		for (index, column) in sheet.columns()?.iter().enumerate() {
			let field = schema.get_field(&column_to_field_name(column)).unwrap();
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
	}

	writer.commit()?;

	tracing::info!("opened index {index:?}");

	Ok(())
}

fn build_sheet_schema(sheet: &Sheet<&str>) -> Result<Schema> {
	let mut schema_builder = Schema::builder();

	// RowID and SubrowID are the only stored fields, search results can be looked up in real excel for the full dataset.
	schema_builder.add_u64_field("row_id", schema::STORED);
	schema_builder.add_u64_field("subrow_id", schema::STORED);

	for column in sheet.columns()? {
		let name = column_to_field_name(&column);

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

			// TODO: not sure how to handle bools... u64 each seems really wasteful
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

fn column_to_field_name(column: &exh::ColumnDefinition) -> String {
	// For packed bool columns, offset alone is not enough to disambiguate a
	// field - add a suffix of the packed bit position.
	use exh::ColumnKind as CK;
	let suffix = match column.kind() {
		CK::PackedBool0 => "_0",
		CK::PackedBool1 => "_1",
		CK::PackedBool2 => "_2",
		CK::PackedBool3 => "_3",
		CK::PackedBool4 => "_4",
		CK::PackedBool5 => "_5",
		CK::PackedBool6 => "_6",
		CK::PackedBool7 => "_7",
		_ => "",
	};

	format!("{}{suffix}", column.offset())
}
