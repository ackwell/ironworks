use anyhow::Result;
use serde::Deserialize;

use super::thaliak;

#[derive(Debug, Deserialize)]
pub struct Config {
	thaliak: thaliak::Config,

	repositories: Vec<String>,
}

pub async fn test(config: Config) -> Result<()> {
	println!("patch booting with config {config:?}");
	let thaliak = thaliak::Provider::new(config.thaliak);
	thaliak
		.patches(config.repositories.first().unwrap().to_string())
		.await?;
	Ok(())
}
