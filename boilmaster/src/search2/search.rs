use std::{
	collections::HashMap,
	hash::{Hash, Hasher},
	path::Path,
	sync::Arc,
};

use anyhow::Result;
use ironworks::{
	excel::{Field, Language, Row, Sheet},
	file::exh,
};
use seahash::SeaHasher;
use tantivy::{directory::MmapDirectory, schema, Document, IndexWriter, UserOperation};
use tokio::{select, sync::RwLock};
use tokio_util::sync::CancellationToken;

use crate::{data::Data, search2::schema::column_field_name, version::VersionKey};

use super::schema::{build_schema, ROW_ID, SHEET_KEY, SUBROW_ID};

pub struct Search {
	indices: RwLock<HashMap<u64, Arc<Index>>>,
}

impl Search {
	pub fn new() -> Self {
		Self {
			indices: Default::default(),
		}
	}

	pub async fn start(&self, cancel: CancellationToken, data: &Data) -> Result<()> {
		let mut receiver = data.subscribe();
		self.temp_ingest(cancel.child_token(), receiver.borrow().clone(), data)
			.await?;

		loop {
			select! {
				Ok(_) = receiver.changed() => {
					self.temp_ingest(cancel.child_token(), receiver.borrow().clone(), data).await?
				}
				_ = cancel.cancelled() => break,
			}
		}

		Ok(())
	}

	// TODO: this needs to go through the version -> sheet -> index abstraction, but for now I'm just testing ideas around ingesting
	async fn temp_ingest(
		&self,
		cancel: CancellationToken,
		versions: Vec<VersionKey>,
		data: &Data,
	) -> Result<()> {
		// TODO: this is obviously dumb, i should store a list of versions that are known to be ingested or in the process of being ingested
		let path = std::fs::canonicalize(".").expect("todo").join("search2");

		let mut import_buckets = HashMap::<u64, Vec<(VersionKey, Sheet<String>)>>::new();

		let mut indicies = self.indices.write().await;
		for version in versions {
			let data_version = data.version(version).expect("todo");
			let excel = data_version.excel();
			let list = excel.list().expect("todo");
			// let list = [
			// 	"Action",
			// 	"Quest",
			// 	"AOZArrangement",
			// 	"quest/024/StmBda202_02471",
			// 	"quest/033/LucKza008_03350",
			// 	"custom/007/CtsMjiAnimalLand_00792",
			// ];

			for sheet_name in list.iter() {
				let sheet = excel.sheet(sheet_name.to_string()).expect("todo");

				let index_key = test_sheet_structure_hash(&sheet).expect("todo");

				let index = indicies
					.entry(index_key)
					.or_insert_with(|| {
						Arc::new(Index::new(&path.join(format!("{index_key:x}")), &sheet))
					})
					.clone();

				import_buckets
					.entry(index_key)
					.or_insert_with(Vec::new)
					.push((version, sheet));

				// 		tracing::info!("s2ingest {}", sheet.name());

				// 		// tokio::task::spawn_blocking(move || {
				// 		index.ingest(version, &sheet);
				// 		// })
				// 		// .await
				// 		// .expect("todo");
			}
		}
		drop(indicies);

		let indicies = self.indices.read().await;
		for (index_key, sheets) in import_buckets {
			let index = indicies.get(&index_key).expect("boom").clone();
			// TODO: this can be semphore'd to limit _buckets_ importing at once
			tokio::task::spawn_blocking(move || {
				index.ingest(&sheets);
			})
			.await
			.expect("boom");
		}

		Ok(())
	}
}

fn test_sheet_structure_hash(sheet: &Sheet<String>) -> Result<u64> {
	// TODO: consider using fixed seeds?
	let mut hasher = SeaHasher::new();
	sheet.kind()?.hash(&mut hasher);

	let mut languages = sheet.languages()?;
	languages.sort_by_key(|language| u8::from(*language));
	languages.hash(&mut hasher);

	// TODO: this encodes the offsets of the columns as well as their kind (and position due to the vec) - technically the actual offset is irrelevant, so would be good to ignore it, but doing so would require decoupling column names from offsets, which I can't do without changes to a lot of stuff in search query resolution. i'm not convinced that different offset layouts for the same structure are going to be common enough to bother.
	// sheet.columns()?.hash(&mut hasher);
	let mut columns = sheet.columns()?;
	columns.sort_by_key(|column| column.offset());
	columns.hash(&mut hasher);

	let hash = hasher.finish();

	Ok(hash)
}

struct Index {
	index: tantivy::Index,
	reader: tantivy::IndexReader,
}

impl Index {
	fn new(path: &Path, sheet: &Sheet<String>) -> Self {
		std::fs::create_dir_all(path).expect("todo");

		let directory = MmapDirectory::open(path).expect("todo");

		let index = match tantivy::Index::exists(&directory).expect("todo") {
			true => tantivy::Index::open(directory).expect("todo"),
			false => {
				let columns = sheet.columns().expect("todo");
				let languages = sheet.languages().expect("todo");
				let schema = build_schema(&columns, &languages);
				tantivy::Index::create(directory, schema, tantivy::IndexSettings::default())
					.expect("todo")
			}
		};

		let reader = index
			.reader_builder()
			.reload_policy(tantivy::ReloadPolicy::Manual)
			.try_into()
			.expect("todo");

		Self { index, reader }
	}

	fn ingest(&self, sheets: &[(VersionKey, Sheet<String>)]) {
		let mut writer = self.index.writer(50 * 1024 * 1024).expect("todo");
		for (version, sheet) in sheets {
			self.ingest_sheet(*version, sheet, &writer);
		}
		writer.commit().expect("todo");
		writer.wait_merging_threads().expect("todo");

		self.reader.reload().expect("boom");
	}

	fn ingest_sheet(&self, version: VersionKey, sheet: &Sheet<String>, writer: &IndexWriter) {
		tracing::info!(%version, name = %sheet.name(), "s2 ingest");

		let columns = sheet.columns().expect("todo");
		let languages = sheet.languages().expect("todo");

		// let mut writer = self.index.writer(50 * 1024 * 1024).expect("todo");
		let schema = self.index.schema();

		let mut documents = HashMap::<(u32, u16), Document>::new();

		for language in languages {
			for row in sheet.with().language(language).iter() {
				let document = documents
					.entry((row.row_id(), row.subrow_id()))
					.or_insert_with(Document::new);
				hydrate_row_document(document, row, &columns, language, &schema).expect("todo");
			}
		}

		// Add the ID fields for all of the recorded documents
		// yikes
		let mut hasher = SeaHasher::new();
		version.hash(&mut hasher);
		sheet.name().hash(&mut hasher);
		let sheet_key = hasher.finish();

		for ((row_id, subrow_id), document) in documents.iter_mut() {
			document.add_u64(schema.get_field(SHEET_KEY).unwrap(), sheet_key);
			document.add_u64(schema.get_field(ROW_ID).unwrap(), (*row_id).into());
			document.add_u64(schema.get_field(SUBROW_ID).unwrap(), (*subrow_id).into());
		}

		// TODO: can probably lift writer out and return an iterator from this if this pattern works
		writer
			.run(documents.into_values().map(UserOperation::Add))
			.expect("todo");

		// writer.commit().expect("todo");
	}
}

fn hydrate_row_document(
	document: &mut Document,
	row: Row,
	columns: &[exh::ColumnDefinition],
	language: Language,
	schema: &schema::Schema,
) -> Result<()> {
	for column in columns {
		let field = schema
			.get_field(&column_field_name(column, language))
			.unwrap();
		let value = row.field(column)?;
		// TODO: this feels pretty repetetive given the column kind schema build - is it avoidable or nah?
		use Field as F;
		match value {
			// TODO: need to make sure the ingested strings don't contain non-string payloads
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

	Ok(())
}
