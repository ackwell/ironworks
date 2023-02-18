use std::{fs, io::Write, path::Path};

use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
use ironworks::zipatch;
use serde::Deserialize;
use tokio::sync::Semaphore;

use super::thaliak;

#[derive(Debug)]
pub struct Patch {
	pub name: String,
	pub url: String,
	// TODO: hash check or size check or something?
}

#[derive(Debug, Deserialize)]
pub struct Config {
	thaliak: thaliak::Config,

	directory: RelativePathBuf,
	concurrency: usize,
	repositories: Vec<String>,
}

// TODO: proper error type
// TODO: this should be versioned (and abstracted, there'll likely need to be a persistence layer for the versioning that gets read from first).
pub async fn wip_build_zipatch(config: Config) -> Result<zipatch::ZiPatch> {
	let provider = thaliak::Provider::new(config.thaliak);

	let target_directory = config.directory.relative();

	let semaphore = Semaphore::new(config.concurrency);

	let pending_repositories = config
		.repositories
		.into_iter()
		.map(|repository| build_repository(&provider, &target_directory, repository, &semaphore));

	let repositories = try_join_all(pending_repositories).await?;

	let zipatch = repositories
		.into_iter()
		.zip(0u8..)
		.fold(zipatch::ZiPatch::new(), |zipatch, (repository, index)| {
			zipatch.with_repository(index, repository)
		});

	Ok(zipatch)
}

async fn build_repository(
	provider: &thaliak::Provider,
	target_directory: &Path,
	repository: String,
	semaphore: &Semaphore,
) -> Result<zipatch::PatchRepository> {
	// Get the path to the directory for this repository, creating it if it does not yet exist.
	let repository_directory = fs::canonicalize(target_directory.join(&repository))?;
	fs::create_dir_all(&repository_directory)?;

	// Get the list of patches expected in this repository, and add in the expected
	// file system path for that patch file.
	let expected_patches = provider
		.patches(repository)
		.await?
		.into_iter()
		.map(|patch| {
			let path = repository_directory.join(&patch.name);
			(patch, path)
		})
		.collect::<Vec<_>>();

	// Any paths that do not exist locally need to be downloaded.
	// TODO: Check size or something here in case there was a partial download.
	let required_patches = expected_patches
		.iter()
		.filter(|(_patch, path)| !path.is_file())
		.collect::<Vec<_>>();

	if !required_patches.is_empty() {
		tracing::info!("missing {} patch files, fetching", required_patches.len());

		let client = reqwest::Client::new();
		// TODO: I'm just immediately fetching everything here - ideally this process would be a bit more multi-stage with like proper UI and everything, but for now this is just MVP to get _something_ local.
		let downloads = required_patches
			.iter()
			.map(|(patch, path)| download_patch(&client, patch, path, semaphore));

		try_join_all(downloads).await?;

		tracing::info!("complete");
	}

	// Download is complete; all the patches exist - build a zipatch repository.
	let repository = zipatch::PatchRepository {
		patches: expected_patches
			.into_iter()
			.map(|(patch, path)| zipatch::Patch {
				name: patch.name,
				path,
			})
			.collect(),
	};

	Ok(repository)
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
