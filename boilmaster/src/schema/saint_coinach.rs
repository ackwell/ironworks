use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use ironworks_schema::{saint_coinach, Schema};
use serde::Deserialize;

use super::{error::Error, provider::Source};

#[derive(Debug, Deserialize)]
pub struct Config {
	remote: Option<String>,
	directory: RelativePathBuf,
}

pub struct SaintCoinach {
	provider: saint_coinach::Provider,
}

impl SaintCoinach {
	pub fn new(config: Config) -> Result<Self> {
		let mut builder = saint_coinach::Provider::with().directory(config.directory.relative());
		if let Some(remote) = config.remote {
			builder = builder.remote(remote);
		}

		Ok(Self {
			provider: builder.build()?,
		})
	}
}

impl Source for SaintCoinach {
	fn version(&self, version: Option<&str>) -> Result<Box<dyn Schema + '_>, Error> {
		// TODO: the schema handler currently has absolutely no means for updating the repository once it's been cloned, so HEAD here will simply be "the position of HEAD at the time the system cloned the repository". Will need to build update mechanisms into stc's provider, and work out how I want to expose that here - it may be a better idea long-term to store the canonical reference for HEAD at the time of the latest update as a field locally?

		let version_id = version.unwrap_or("HEAD");
		let version = self.provider.version(version_id).map_err(|error| {
			use ironworks_schema::Error as SE;
			use ironworks_schema::ErrorValue as SEV;
			match error {
				SE::NotFound(SEV::Version(_)) => Error::InvalidVersion(version_id.into()),
				other => Error::Failure(other.into()),
			}
		})?;

		Ok(Box::new(version))
	}
}
