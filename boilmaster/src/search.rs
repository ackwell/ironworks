use std::{
	collections::HashMap,
	env::current_exe,
	fs,
	path::{Path, PathBuf},
};

use anyhow::Result;
use ironworks::{
	excel::{Excel, Field, Row, Sheet},
	file::exh,
};
use tantivy::{
	self,
	collector::TopDocs,
	directory::MmapDirectory,
	query::QueryParser,
	schema::{self, Schema},
	Document, IndexSettings, ReloadPolicy,
};

use crate::data::Data;

pub struct Search {
	path: PathBuf,

	// TODO: this should be a map of version keys to search::Version instances
	temp_version: Option<Version>,
}

impl Search {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		// TODO: configurable directory, this shouldn't be touching current exe at all
		let path = current_exe()
			.expect("could not resolve current executable")
			.parent()
			.expect("current path has no parent")
			.join("search");

		Self {
			path,
			temp_version: None,
		}
	}

	// TODO: who "owns" data ref? - i don't think search needs data outside the init step?
	// is there any point in this being seperate from new(), really?
	pub fn initialize(&mut self, data: &Data) -> Result<()> {
		// ... do... i want to pass this shit to version and let it pin down, or do i want to pin down here and pass shit down to version? like i guess if anything the index needs the sheet name so it can lazy init an excel sheet for ingest so keeping that up for the other shit makes sense?
		let version = Version::new("TODO VERSION", &self.path, data)?;

		// TODO: TEMP testing shit
		version.search("summon");

		// TODO: I'm tempted to say that indexing versions should be lazy but... idk. check how long it takes to index a full gamever - if it's a notable duration on my computer it'll probably be glacial on a server.
		self.temp_version = Some(version);

		Ok(())
	}
}

pub struct Version {
	// this should probably be the canonical version struct or something?
	version: String,

	// some tables like custom/ and quest/ are going to have a name that isn't a valid file path - what do we want to use for the keys here, and should it be considered the canonical name for indices?
	indices: HashMap<String, Index>,
}

impl Version {
	fn new(version: &str, search_path: &Path, data: &Data) -> Result<Self> {
		let path = search_path.join(version);
		// TODO: THIS SHOULD BE USING THE VERSION FROM THE ARGUMENTS. Doing so will need fixing up on the data side; which in turn will need a structured version system. should probably add that hey.
		let data_version = data.version(None);
		let excel = data_version.excel();

		// TODO: build an index for every sheet
		let sheets = ["Action"];

		let mut indices = HashMap::new();
		for name in sheets {
			// TODO: if this is async; how do i run them all at the same time?
			let index = Index::new(name, &path, excel)?;
			indices.insert(name.to_string(), index);
		}

		Ok(Self {
			version: version.to_string(),
			indices,
		})
	}

	// TODO: index specifier?
	// TODO: non-string-query filters
	fn search(&self, query: &str) {
		// TODO: this should combine across multiple indicies in some score-centric way?
		let x = self
			.indices
			.values()
			.map(|index| index.search(query))
			.collect::<Vec<_>>();

		tracing::debug!("search result: {x:#?}");
	}
}

// TODO: should this be public?
struct Index {
	sheet_name: String,
	// Do i actually need a reference to the index at all?
	index: tantivy::Index,
	reader: tantivy::IndexReader,
}

impl Index {
	fn new(sheet_name: &str, version_path: &Path, excel: &Excel) -> Result<Self> {
		let path = version_path.join(sheet_name);
		fs::create_dir_all(&path)?;
		let directory = MmapDirectory::open(&path)?;

		// TODO: this should likely be the part of the path that gets joined onto the base or similar
		let index = match tantivy::Index::exists(&directory)? {
			// NOTE: this assumes that the schema will be effectively immutable
			true => {
				tracing::debug!("Opening index {path:?}");
				tantivy::Index::open(directory)?
			}
			false => {
				tracing::debug!("Building index {path:?}");
				let sheet = excel.sheet(sheet_name)?;
				// TODO: this should do... something. retry? i don't know. if any step of ingestion fails. A failed ingest is pretty bad.
				// TODO: maybe rayon for executing the blocking init?
				ingest_sheet(&sheet, directory)
					.expect("TODO: error handling for ingestion failures")
			}
		};

		let reader = index
			.reader_builder()
			// TODO: this is set to manual 'cus technically an index is never updated in this setup. is that sane? i dunno?
			.reload_policy(ReloadPolicy::Manual)
			.try_into()?;

		Ok(Self {
			sheet_name: sheet_name.to_string(),
			index,
			reader,
		})
	}

	fn search(&self, query_string: &str) -> Result<Vec<(f32, u64)>> {
		let searcher = self.reader.searcher();

		// this is cloning the schema every search - should i just store a single copy of it on the struct?
		let schema = self.index.schema();

		// so immediate complication to deal with; need to specify the fields to search if the user (lmao as if) doesn't specify any. we... techncially want to search _every_thing. or, at least every string thing? idfk. all strings makes sense i guess?
		let fields = schema
			.fields()
			.filter(|(_field, entry)| matches!(entry.field_type(), schema::FieldType::Str(_)))
			.map(|(field, _entry)| field)
			.collect::<Vec<_>>();

		let query_parser = QueryParser::for_index(&self.index, fields);
		let query = query_parser.parse_query(query_string)?;

		let top_docs = searcher.search(&query, &TopDocs::with_limit(100))?;
		let todo_result = top_docs
			.into_iter()
			.map(|(score, doc_address)| {
				let doc = searcher.doc(doc_address).expect(
					"TODO: error handling. is there any reasonable expectation this will fail?",
				);
				// TODO: error handling; schema field lookup can probably omve outside loop
				let id = doc
					.get_first(schema.get_field("row_id").unwrap())
					.unwrap()
					.as_u64()
					.unwrap();
				// todo: this should probably convert rowid to u32, and also obtain u16 subrow id i assume?
				(score, id)
			})
			.collect::<Vec<_>>();

		Ok(todo_result)
	}
}

fn ingest_sheet(sheet: &Sheet<&str>, directory: MmapDirectory) -> Result<tantivy::Index> {
	let columns = sheet.columns()?;

	let index = tantivy::Index::create(
		directory,
		build_sheet_schema(&columns)?,
		IndexSettings::default(),
	)?;

	// Allocating 50mb for writer ingestion.
	// TODO: Is this per index? A bulk version ingestion might be problematic @ 50mb/ea, test this behavior.
	// TODO: probably should be configurable anyway.
	let mut writer = index.writer(50 * 1024 * 1024)?;

	// TODO: if there's any failures at all (i.e. iw read errors) during ingestion, the writer should be rolled back to ensure a theoretical retry is able to work on a clean deck.
	for row in sheet.iter() {
		let document = build_row_document(row, &columns, &index.schema())?;

		// this can block; which suggests to me that at minimum, ingesting should be done on the side.
		writer.add_document(document)?;
	}

	writer.commit()?;

	Ok(index)
}

fn build_sheet_schema(columns: &[exh::ColumnDefinition]) -> Result<Schema> {
	let mut schema_builder = Schema::builder();

	// RowID and SubrowID are the only stored fields, search results can be looked up in real excel for the full dataset.
	schema_builder.add_u64_field("row_id", schema::STORED);
	schema_builder.add_u64_field("subrow_id", schema::STORED);

	for column in columns {
		let name = column_to_field_name(column);

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

fn build_row_document(
	row: Row,
	columns: &[exh::ColumnDefinition],
	schema: &Schema,
) -> Result<Document> {
	let mut document = Document::new();

	document.add_u64(schema.get_field("row_id").unwrap(), (*row.row_id()).into());
	document.add_u64(
		schema.get_field("subrow_id").unwrap(),
		(*row.subrow_id()).into(),
	);

	for (index, column) in columns.iter().enumerate() {
		let field = schema.get_field(&column_to_field_name(column)).unwrap();
		// TODO: this would really value .field(impl intocolumn) or similar
		let value = row.field(index)?;
		// TODO: this feels pretty repetetive given the column kind schema build - is it avoidable or nah?
		use Field as F;
		match value {
			F::String(value) => document.add_text(field, value),

			F::I8(value) => document.add_i64(field, value.into()),
			F::I16(value) => document.add_i64(field, value.into()),
			F::I32(value) => document.add_i64(field, value.into()),
			F::I64(value) => document.add_i64(field, value),

			F::U8(value) => document.add_u64(field, value.into()),
			F::U16(value) => document.add_u64(field, value.into()),
			F::U32(value) => document.add_u64(field, value.into()),
			F::U64(value) => document.add_u64(field, value),

			F::F32(value) => document.add_f64(field, value.into()),

			F::Bool(value) => document.add_u64(field, value.into()),
		}
	}

	Ok(document)
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
