use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};
use futures::future::try_join_all;
use ironworks::{
	excel::{Excel, Language},
	sqpack::SqPack,
	zipatch, Ironworks,
};
use serde::Deserialize;

use crate::version::PatchList;

use super::{language::LanguageString, patch};

#[derive(Debug, Deserialize)]
pub struct Config {
	patch: patch::Config,

	language: LanguageString,
}

pub struct Data {
	default_language: Language,

	// Root ZiPatch instance, acts as a LUT cache
	zipatch: zipatch::ZiPatch,

	patcher: patch::Patcher,

	versions: RwLock<HashMap<String, Arc<Version>>>,
}

impl Data {
	pub fn new(config: Config) -> Self {
		Data {
			default_language: config.language.into(),
			zipatch: zipatch::ZiPatch::new().with_persisted_lookups(),
			patcher: patch::Patcher::new(config.patch),
			versions: Default::default(),
		}
	}

	pub fn default_language(&self) -> Language {
		self.default_language
	}

	pub async fn prepare_version(&self, version_name: String, patch_list: PatchList) -> Result<()> {
		// Start getting paths for all the patches required for this version, downloading if required.
		let pending_repositories = patch_list
			.into_iter()
			.map(|(repository, patches)| async move {
				let mut patch_paths = self.patcher.patch_paths(&repository, &patches).await?;

				let zipatch_patches = patches
					.into_iter()
					.map(|patch| {
						let zipatch_patch = zipatch::Patch {
							path: patch_paths.remove(&patch.name).ok_or_else(|| {
								anyhow!("patch {} missing in patcher path response", patch.name)
							})?,
							name: patch.name,
						};
						Ok(zipatch_patch)
					})
					.collect::<Result<Vec<_>>>()?;

				Ok::<_, anyhow::Error>(zipatch::PatchRepository {
					patches: zipatch_patches,
				})
			});

		// Ensure that all patches are ready.
		let repositories = try_join_all(pending_repositories).await?;

		// Build a zipatch view into the patches.
		let view = repositories
			.into_iter()
			.zip(0u8..)
			.fold(self.zipatch.view(), |builder, (repository, index)| {
				builder.with_repository(index, repository)
			})
			.build();

		// Build a version and save it out to the struct.
		let version = Version::new(view);
		self.versions
			.write()
			.expect("poisoned")
			.insert(version_name, Arc::new(version));

		Ok(())
	}

	pub fn version(&self, version: &str) -> Option<Arc<Version>> {
		self.versions
			.read()
			.expect("poisoned")
			.get(version)
			.cloned()
	}
}

pub struct Version {
	excel: Arc<Excel<'static>>,
}

impl Version {
	fn new(view: zipatch::View) -> Self {
		let ironworks = Ironworks::new().with_resource(SqPack::new(view));
		let excel = Excel::new(Arc::new(ironworks));
		Self {
			excel: Arc::new(excel),
		}
	}

	pub fn excel(&self) -> Arc<Excel<'static>> {
		Arc::clone(&self.excel)
	}
}
