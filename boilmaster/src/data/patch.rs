use std::{
	collections::HashMap,
	fs,
	io::Write,
	path::{Path, PathBuf},
};

use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
use serde::Deserialize;
use tokio::sync::Semaphore;

use crate::version::Patch;

#[derive(Debug, Deserialize)]
pub struct Config {
	directory: RelativePathBuf,
	concurrency: usize,
}

pub struct Patcher {
	directory: PathBuf,
	semaphore: Semaphore,
}

impl Patcher {
	pub fn new(config: Config) -> Self {
		Self {
			directory: config.directory.relative(),
			semaphore: Semaphore::new(config.concurrency),
		}
	}

	// TODO: proper error type
	pub async fn patch_paths(
		&self,
		repository_name: &str,
		patches: &[Patch],
	) -> Result<HashMap<String, PathBuf>> {
		// TODO: This seems silly to run on a regular basis given it'll only actually do something like 4 times ever.
		let repository_directory = self.directory.join(repository_name);
		fs::create_dir_all(&repository_directory)?;

		// Pair patches with their FS path.
		let expected_patches = patches
			.iter()
			.map(|patch| (patch, repository_directory.join(&patch.name)))
			.collect::<Vec<_>>();

		// Any paths that do not exist locally, or are the incorrect size, need to be (re-)downloaded.
		let required_patches = expected_patches
			.iter()
			.filter(|(patch, path)| {
				let Ok(metadata) = path.metadata() else {
					return true
				};

				let size_matches = metadata.len() == patch.size;

				if !size_matches {
					tracing::warn!(
						"patch {} size mismatch, re-fetching (expected {}, got {})",
						patch.name,
						patch.size,
						metadata.len()
					);
				}

				!path.is_file() || !size_matches
			})
			.collect::<Vec<_>>();

		// If there are patches that need to be downloaded, go and actually do that.
		if !required_patches.is_empty() {
			tracing::info!("missing {} patch files, fetching", required_patches.len());

			let client = reqwest::Client::new();
			let downloads = required_patches
				.into_iter()
				.map(|(patch, path)| download_patch(&client, patch, path, &self.semaphore));

			try_join_all(downloads).await?;
		}

		// Build the final path mapping.
		let path_map = expected_patches
			.into_iter()
			.map(|(patch, path)| (patch.name.clone(), path))
			.collect::<HashMap<_, _>>();

		Ok(path_map)
	}
}

async fn download_patch(
	client: &reqwest::Client,
	patch: &Patch,
	target_path: &Path,
	semaphore: &Semaphore,
) -> Result<()> {
	let permit = semaphore.acquire().await.unwrap();

	tracing::info!("downloading patch {}", patch.name);

	// Create the target file before opening any connections.
	let mut target_file = fs::File::create(target_path)?;

	// Initiate a request to the patch file
	let mut response = client.get(&patch.url).send().await?;
	let content_length = response.content_length().ok_or_else(|| {
		anyhow::anyhow!("Could not find patch content length for {}.", patch.name)
	})?;

	// Stream the file to disk.
	let mut position = 0;
	let mut last_report = 0.;
	while let Some(chunk) = response.chunk().await? {
		// this is probably blocking - is it worth doing some of this on a spawn_blocking?
		target_file.write_all(&chunk)?;

		position += u64::try_from(chunk.len()).unwrap();
		let report_pos = f64::round((position as f64 / content_length as f64) * 20.) * 5.;
		if report_pos > last_report {
			tracing::debug!(
				"{}: {position}/{content_length} ({report_pos}%)",
				patch.name
			);
			last_report = report_pos;
		}
	}

	drop(permit);

	Ok(())
}
