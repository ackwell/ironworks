use std::{
	collections::{BTreeMap, HashMap},
	fs,
	path::PathBuf,
	sync::RwLock,
};

use super::{
	key::VersionKey,
	patch::{Patch, PatchStore},
	persist::JsonFile,
	thaliak,
	version::Version,
};
use anyhow::{anyhow, Context, Result};
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
use serde::Deserialize;
use tokio::{select, sync::watch, time};
use tokio_util::sync::CancellationToken;

pub type PatchList = Vec<(String, Vec<Patch>)>;

#[derive(Debug, Deserialize)]
pub struct Config {
	thaliak: thaliak::Config,

	interval: u64,
	directory: RelativePathBuf,
	repositories: Vec<String>,
}

// TODO: might want to make version names many:one so i.e. "6.38" can also be "latest"
//       is it worth adding multi-tag support for that or should i just have a seperate latest pointer
const LATEST_TAG: &str = "latest";

#[derive(Debug)]
pub struct Manager {
	thaliak: thaliak::Provider,

	update_interval: u64,

	file: JsonFile,
	directory: PathBuf,

	channel: watch::Sender<Vec<VersionKey>>,

	repositories: Vec<String>,
	patches: HashMap<String, RwLock<PatchStore>>,
	versions: RwLock<HashMap<VersionKey, Version>>,
	version_names: RwLock<HashMap<String, VersionKey>>,
}

impl Manager {
	pub fn new(config: Config) -> Result<Self> {
		let directory = config.directory.relative();

		// Build a patch store for each repository - repositories are stable within
		// a given running instance, so doing this eagerly is safe.
		let patches = config
			.repositories
			.iter()
			.map(|repository_name| {
				(
					repository_name.to_string(),
					RwLock::new(PatchStore::new(&directory, repository_name)),
				)
			})
			.collect::<HashMap<_, _>>();

		let (sender, _receiver) = watch::channel(vec![]);

		let manager = Self {
			thaliak: thaliak::Provider::new(config.thaliak),
			update_interval: config.interval,
			file: JsonFile::new(directory.join("versions.json")),
			directory,
			channel: sender,
			repositories: config.repositories,
			patches,
			versions: Default::default(),
			version_names: Default::default(),
		};

		// Eagerly kick off an initial hydration from disk.
		manager.hydrate()?;

		Ok(manager)
	}

	/// Subscribe to changes to the version list.
	pub fn subscribe(&self) -> watch::Receiver<Vec<VersionKey>> {
		self.channel.subscribe()
	}

	/// Resolve a version name to it's key. If no version is specified, the version marked as latest will be returned, if any exists.
	// TODO: remove the fallback logic from here, push it up to the consumer, akin to schema specifier?
	pub fn resolve(&self, name: Option<&str>) -> Option<VersionKey> {
		self.version_names
			.read()
			.expect("poisoned")
			.get(name.unwrap_or(LATEST_TAG))
			.cloned()
	}

	/// Get a patch list for a given version.
	pub fn patch_list(&self, key: &VersionKey) -> Result<PatchList> {
		// Fetch the requested version.
		let versions = self.versions.read().expect("poisoned");
		let version = versions
			.get(key)
			.with_context(|| format!("unknown version {key}"))?;

		// TODO: A version made on repository list [a, b] will create a patch list [a] on an updated repository list of [a, X, b]. I'm not convinced that's a problem.
		let patch_list = self
			.repositories
			.iter()
			.map_while(|repository_name| {
				// Get the patch store for this repository.
				let patches = self.patches.get(repository_name)?.read().expect("poisoned");
				let patch_names = version.patches().get(repository_name)?;

				// Resolve the patches for this version against the patch store.
				let list = patch_names
					.iter()
					.map(|patch_name| {
						patches.patch(patch_name).with_context(|| {
							format!("missing patch metadata for {repository_name} {patch_name}")
						})
					})
					.collect::<Result<Vec<_>>>()
					.map(|list| (repository_name.clone(), list));

				Some(list)
			})
			.collect::<Result<Vec<_>>>()?;

		Ok(patch_list)
	}

	/// Hydrate data from persisted files.
	fn hydrate(&self) -> Result<()> {
		// Hydrate all the repository patch stores.
		for patch_store in self.patches.values() {
			patch_store.write().expect("poisoned").hydrate()?;
		}

		// Pull in the list of every known version. Keys here are the version keys,
		// inverse to what we use in-memory.
		let all_versions = self.file.read::<HashMap<VersionKey, Vec<String>>>()?;

		let mut version_names = self.version_names.write().expect("poisoned");
		let mut versions = self.versions.write().expect("poisoned");

		for (version_key, names) in all_versions {
			// Save the names out.
			for name in names {
				version_names.insert(name, version_key.clone());
			}

			// Build a version representation and hydrate it from disk.
			let mut version = Version::new(&self.directory, &version_key);
			version.hydrate()?;
			versions.insert(version_key, version);
		}

		drop(version_names);
		drop(versions);

		// Broadcast the initial state of the version list.
		self.broadcast_version_list();

		Ok(())
	}

	/// Start the service.
	pub async fn start(&self, cancel: CancellationToken) -> Result<()> {
		let mut interval = time::interval(time::Duration::from_secs(self.update_interval));
		interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

		loop {
			select! {
				_ = interval.tick() => {},
				_ = cancel.cancelled() => { break }
			}

			if let Err(error) = self.update().await {
				tracing::error!(%error, "update failed");
			}
		}

		Ok(())
	}

	/// Update local data from upstream sources.
	pub async fn update(&self) -> Result<()> {
		tracing::info!("checking for version updates");

		// Ensure that the persistance folder exists before trying to write anything to it.
		fs::create_dir_all(&self.directory)?;

		// Fetch the patch lists for each of the repositories.
		let pending_repository_patches = self
			.repositories
			.iter()
			.map(|repository_name| self.update_repository_patches(repository_name));

		let repository_patches = try_join_all(pending_repository_patches).await?;

		// Get the latest patches for the updated patch lists and use to build the current version key.
		let latest_patches = repository_patches
			.iter()
			.zip(&self.repositories)
			.map(|(patches, repository_name)| {
				patches
					.last()
					.ok_or_else(|| anyhow!("no patches found for repository {repository_name}"))
			})
			.collect::<Result<Vec<_>>>()?;

		let key = VersionKey::from_latest_patches(&latest_patches);

		// Merge the versions with the repository names and update the version with
		// it, creating a new version if it's not been seen before.
		let value = self
			.repositories
			.clone()
			.into_iter()
			.zip(repository_patches)
			.collect::<HashMap<_, _>>();

		let mut versions = self.versions.write().expect("poisoned");
		let version = versions
			.entry(key.clone())
			.or_insert_with(|| Version::new(&self.directory, &key));

		version.update(value)?;
		drop(versions);

		// TEMP: for now, setting the latest sigil to always point to the most recent version key. Don't merge this, hey?
		let mut vn_temp = self.version_names.write().expect("poisoned");
		vn_temp.insert(LATEST_TAG.to_string(), key);
		drop(vn_temp);

		// Build the full version listing for persisting.
		let versions = self.versions.read().expect("poisoned");
		let version_names = self.version_names.read().expect("poisoned");

		let mut all_versions = versions
			.keys()
			.map(|key| (key, vec![]))
			.collect::<BTreeMap<_, _>>();

		for (name, version) in version_names.iter() {
			all_versions
				.entry(version)
				.or_insert_with(Vec::new)
				.push(name);
		}

		self.file.write(&all_versions)?;

		// Broadcast any changes to the version list from this update.
		self.broadcast_version_list();

		Ok(())
	}

	async fn update_repository_patches(&self, repository_name: &str) -> Result<Vec<String>> {
		// Get the latest patches for this repository.
		let patches = self.thaliak.patches(repository_name.to_string()).await?;

		// Grab the list of patch names for later use.
		let patch_names = patches
			.iter()
			.map(|patch| patch.name.to_string())
			.collect::<Vec<_>>();

		// Save the patch data to the repository's patch store.
		self.patches
			.get(repository_name)
			.ok_or_else(|| anyhow!("missing patch store for repository {repository_name}"))?
			.write()
			.expect("posioned")
			.update(patches)?;

		Ok(patch_names)
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
