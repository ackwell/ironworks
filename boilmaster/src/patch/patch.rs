use std::{
	collections::HashSet,
	fs,
	path::{Path, PathBuf},
};

use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
use serde::Deserialize;

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
	repositories: Vec<String>,
}

// TODO: proper error type
pub async fn test(config: Config) -> Result<()> {
	println!("patch booting with config {config:?}");
	let provider = thaliak::Provider::new(config.thaliak);

	let target_directory = config.directory.relative();

	let repositories = config
		.repositories
		.into_iter()
		.map(|repository| check_repository(&provider, &target_directory, repository));

	let todo = try_join_all(repositories).await?;

	Ok(())
}

async fn check_repository(
	provider: &thaliak::Provider,
	target_directory: &Path,
	repository: String,
) -> Result<()> {
	let repository_directory = fs::canonicalize(target_directory.join(&repository))?;

	// todo get rid of that clone
	let expected_patches = provider.patches(repository.clone()).await?;

	let current_patches = current_patches(&repository_directory)?;

	let required_patches = expected_patches
		.iter()
		.filter(|x| !current_patches.contains(&x.name))
		.collect::<Vec<_>>();

	println!("repository {repository} requires:\n{required_patches:#?}");

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
