use std::{
	collections::HashMap,
	hash::{Hash, Hasher},
	path::Path,
	sync::RwLock,
};

use anyhow::Result;
use ironworks::excel::Sheet;
use seahash::SeaHasher;
use tantivy::directory::MmapDirectory;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{data::Data, version::VersionKey};

use super::schema::build_schema;

pub struct Search {
	indices: RwLock<HashMap<u64, Index>>,
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
		let mut indicies = self.indices.write().expect("poisoned");
		for version in versions {
			let data_version = data.version(version).expect("todo");
			let excel = data_version.excel();
			let list = excel.list().expect("todo");

			for sheet_name in list.iter() {
				let sheet = excel.sheet(sheet_name.to_string()).expect("todo");

				let index_key = test_sheet_structure_hash(&sheet).expect("todo");

				let index = indicies
					.entry(index_key)
					.or_insert_with(|| Index::new(&path.join(format!("{index_key:x}")), &sheet));
			}
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
}
