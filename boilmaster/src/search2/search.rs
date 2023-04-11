use anyhow::{Context, Result};
use itertools::Itertools;
use serde::Deserialize;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{data::Data, version::VersionKey};

use super::tantivy;

#[derive(Debug, Deserialize)]
pub struct Config {
	tantivy: tantivy::Config,
}

pub struct Search {
	provider: tantivy::Provider,
}

impl Search {
	pub fn new(config: Config) -> Self {
		Self {
			provider: tantivy::Provider::new(config.tantivy),
		}
	}

	pub async fn start(&self, cancel: CancellationToken, data: &Data) -> Result<()> {
		let mut receiver = data.subscribe();
		self.ingest(cancel.child_token(), receiver.borrow().clone(), data)
			.await?;

		loop {
			select! {
				Ok(_) = receiver.changed() => {
					self.ingest(cancel.child_token(), receiver.borrow().clone(), data).await?
				}
				_ = cancel.cancelled() => break,
			}
		}

		Ok(())
	}

	async fn ingest(
		&self,
		cancel: CancellationToken,
		versions: Vec<VersionKey>,
		data: &Data,
	) -> Result<()> {
		// Get a list of all sheets in the provided versions.
		// TODO: This has more `.collect`s than i'd like, but given it's a fairly cold path, probably isn't a problem.
		let sheets = versions
			.into_iter()
			.map(|version| -> Result<_> {
				let data_version = data.version(version).with_context(|| {
					format!("version {version} announced for ingestion but not provided")
				})?;
				let excel = data_version.excel();
				let list = excel.list()?;

				list.iter()
					.map(|sheet_name| Ok((version, excel.sheet(sheet_name.to_string())?)))
					.collect::<Result<Vec<_>>>()
			})
			.flatten_ok()
			.collect::<Result<Vec<_>>>()?;

		// Fire off the ingestion in the provider.
		self.provider.ingest(cancel, sheets).await?;

		Ok(())
	}
}
