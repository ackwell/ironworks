use std::{
	collections::{HashMap, HashSet},
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
use tokio::{select, sync::watch};
use tokio_util::sync::CancellationToken;

use crate::version::{self, VersionKey};

use super::{language::LanguageString, patch};

#[derive(Debug, Deserialize)]
pub struct Config {
	patch: patch::Config,

	language: LanguageString,
}

pub struct Data {
	default_language: Language,

	channel: watch::Sender<Vec<VersionKey>>,

	// Root ZiPatch instance, acts as a LUT cache
	zipatch: zipatch::ZiPatch,

	patcher: patch::Patcher,

	versions: RwLock<HashMap<VersionKey, Arc<Version>>>,
}

impl Data {
	pub fn new(config: Config) -> Self {
		let (sender, _receiver) = watch::channel(vec![]);

		Data {
			default_language: config.language.into(),
			channel: sender,
			zipatch: zipatch::ZiPatch::new().with_persisted_lookups(),
			patcher: patch::Patcher::new(config.patch),
			versions: Default::default(),
		}
	}

	pub fn default_language(&self) -> Language {
		self.default_language
	}

	pub fn subscribe(&self) -> watch::Receiver<Vec<VersionKey>> {
		self.channel.subscribe()
	}

	pub async fn start(&self, cancel: CancellationToken, version: &version::Manager) -> Result<()> {
		let mut receiver = version.subscribe();
		self.prepare_new_versions(version, receiver.borrow().clone())
			.await?;

		loop {
			select! {
				Ok(_) = receiver.changed() => {
					self.prepare_new_versions(version, receiver.borrow().clone()).await?
				}
				_ = cancel.cancelled() => break,
			}
		}

		Ok(())
	}

	async fn prepare_new_versions(
		&self,
		version: &version::Manager,
		versions: Vec<VersionKey>,
	) -> Result<()> {
		let known_keys = self
			.versions
			.read()
			.expect("poisoned")
			.keys()
			.cloned()
			.collect::<HashSet<_>>();

		let prepares = versions
			.into_iter()
			.filter(|key| !known_keys.contains(key))
			.map(|key| self.prepare_version(version, key));

		try_join_all(prepares).await?;

		Ok(())
	}

	// TODO: should this use an explicit cancellation token?
	async fn prepare_version(
		&self,
		version: &version::Manager,
		version_key: VersionKey,
	) -> Result<()> {
		let patch_list = version.patch_list(&version_key)?;

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
			.insert(version_key, Arc::new(version));

		// Broadcast the update.
		// NOTE: This is performed after each version rather than when all versions
		// are complete to allow other services to begin processing an early-completing
		// version before the full patch process is complete.
		self.broadcast_version_list();

		Ok(())
	}

	pub fn version(&self, version: &VersionKey) -> Option<Arc<Version>> {
		self.versions
			.read()
			.expect("poisoned")
			.get(version)
			.cloned()
	}

	fn broadcast_version_list(&self) {
		let versions = self.versions.read().expect("poisoned");
		let keys = versions.keys().cloned().collect::<Vec<_>>();

		self.channel.send_if_modified(|value| {
			if &keys != value {
				*value = keys;
				return true;
			}

			false
		});
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
