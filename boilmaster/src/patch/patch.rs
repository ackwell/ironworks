use std::{
	collections::HashSet,
	fs,
	io::Write,
	path::{Path, PathBuf},
};

use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
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
pub async fn test(config: Config) -> Result<()> {
	println!("patch booting with config {config:?}");
	let provider = thaliak::Provider::new(config.thaliak);

	let target_directory = config.directory.relative();

	let semaphore = Semaphore::new(config.concurrency);

	let repositories = config
		.repositories
		.into_iter()
		.map(|repository| check_repository(&provider, &target_directory, repository, &semaphore));

	let todo = try_join_all(repositories).await?;

	Ok(())
}

async fn check_repository(
	provider: &thaliak::Provider,
	target_directory: &Path,
	repository: String,
	semaphore: &Semaphore,
) -> Result<()> {
	let repository_directory = fs::canonicalize(target_directory.join(&repository))?;

	let expected_patches = provider.patches(repository).await?;

	let current_patches = current_patches(&repository_directory)?;

	let required_patches = expected_patches
		.iter()
		.filter(|patch| !current_patches.contains(&patch.name))
		.collect::<Vec<_>>();

	if !required_patches.is_empty() {
		tracing::info!("missing {} patch files, fetching", required_patches.len());

		let client = reqwest::Client::new();
		// TODO: I'm just immediately fetching everything here - ideally this process would be a bit more multi-stage with like proper UI and everything, but for now this is just MVP to get _something_ local.
		let downloads = required_patches
			.iter()
			.map(|patch| download_patch(&client, &repository_directory, patch, semaphore));

		try_join_all(downloads).await?;

		tracing::info!("complete");
	}

	Ok(())
}

// TODO: how should i handle partial downloads &c
fn current_patches(repository_directory: &Path) -> Result<HashSet<String>> {
	// Make sure that the directory exists.
	fs::create_dir_all(repository_directory)?;

	// Get a list of all patch files in the directory
	// TODO: is it sane to ignore stuff like this .ok? ergh.
	// ASSUMPTION: BM has full control over this directory - it doesn't check if someonee's put something dumb in there.
	let current_patches = fs::read_dir(repository_directory)?
		.filter_map(|entry| {
			let file_name = PathBuf::from(entry.ok()?.file_name());
			let patch = file_name.file_stem()?.to_str()?.to_string();
			Some(patch)
		})
		.collect::<HashSet<_>>();

	Ok(current_patches)
}

async fn download_patch(
	client: &reqwest::Client,
	repository_directory: &Path,
	patch: &Patch,
	semaphore: &Semaphore,
) -> Result<()> {
	let permit = semaphore.acquire().await.unwrap();

	tracing::debug!("downloading patch {}", patch.name);

	// Create the target file before opening any connections.
	let path = repository_directory.join(format!("{}.patch", &patch.name));
	let mut target_file = fs::File::create(path)?;

	// Initiate a request to the patch file
	let mut response = client.get(&patch.url).send().await?;
	let content_length = response.content_length().ok_or_else(|| {
		anyhow::anyhow!("Could not find patch content length for {}.", patch.name)
	})?;

	// Stream the file to disk.
	let mut position = 0;
	while let Some(chunk) = response.chunk().await? {
		// this is probably blocking - is it worth doing some of this on a spawn_blocking?
		target_file.write_all(&chunk)?;

		position += u64::try_from(chunk.len()).unwrap();
		tracing::debug!("{}: {position}/{content_length}", patch.name);
	}

	drop(permit);

	Ok(())
}
