use super::thaliak;
use anyhow::Result;
use futures::future::try_join_all;
use serde::Deserialize;

#[derive(Debug)]
pub struct Patch {
	pub name: String,
	pub url: String,
	pub size: u64,
}

pub type PatchList = Vec<(String, Vec<Patch>)>;

#[derive(Debug, Deserialize)]
pub struct Config {
	thaliak: thaliak::Config,

	repositories: Vec<String>,
}

pub async fn wip_get_patch_list(config: Config) -> Result<PatchList> {
	let provider = thaliak::Provider::new(config.thaliak);

	let a = config
		.repositories
		.into_iter()
		.map(|repository_name| get_repository_patches(&provider, repository_name));

	try_join_all(a).await
}

async fn get_repository_patches(
	provider: &thaliak::Provider,
	repository_name: String,
) -> Result<(String, Vec<Patch>)> {
	Ok((
		repository_name.clone(),
		provider.patches(repository_name).await?,
	))
}
