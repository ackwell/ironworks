use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	fs,
	hash::{Hash, Hasher},
	io,
	path::{Path, PathBuf},
	sync::RwLock,
};

use super::thaliak;
use anyhow::{anyhow, Result};
use figment::value::magic::RelativePathBuf;
use fs4::FileExt;
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
	pub name: String,
	pub url: String,
	pub size: u64,
}

pub type PatchList = Vec<(String, Vec<Patch>)>;

#[derive(Debug, Deserialize)]
pub struct Config {
	thaliak: thaliak::Config,

	directory: RelativePathBuf,
	repositories: Vec<String>,
}

pub async fn wip_get_patch_list(config: Config) -> Result<PatchList> {
	tracing::debug!("patchlist");
	let vt = VersioningThing::new(config);
	tracing::debug!("hydrate");
	vt.hydrate()?;
	tracing::debug!("update");
	vt.update().await?;
	tracing::debug!("done");

	// temp shit to make the rest of the system work
	// let provider = thaliak::Provider::new(config.thaliak);
	let provider = &vt.thaliak;

	// let a = config
	let a = vt
		.repositories
		.into_iter()
		.map(|repository_name| get_repository_patches(provider, repository_name));

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

#[derive(Debug)]
struct VersioningThing {
	thaliak: thaliak::Provider,
	directory: PathBuf,

	repositories: Vec<String>,
	patches: HashMap<String, RwLock<PatchStore>>,
	versions: RwLock<HashMap<String, Version>>,
	version_names: RwLock<HashMap<String, String>>,
}

impl VersioningThing {
	fn new(config: Config) -> Self {
		let directory = config.directory.relative();

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

		Self {
			thaliak: thaliak::Provider::new(config.thaliak),
			directory,
			repositories: config.repositories,
			patches,
			versions: Default::default(),
			version_names: Default::default(),
		}
	}

	// read from disk anything that already exists
	fn hydrate(&self) -> Result<()> {
		for patch_store in self.patches.values() {
			patch_store.write().expect("poisoned").hydrate()?;
		}

		// TODO: hydrate versions - will need a "top level" version list to do this.
		let file = match fs::File::open(self.directory.join("versions.json")) {
			Ok(file) => file,
			Err(error) => match error.kind() {
				io::ErrorKind::NotFound => return Ok(()),
				_ => Err(error)?,
			},
		};
		file.lock_shared()?;

		let all_versions: HashMap<String, Option<String>> = serde_json::from_reader(file)?;

		let mut version_names = self.version_names.write().expect("poisoned");
		let mut versions = self.versions.write().expect("poisoned");

		for (version_key, maybe_name) in all_versions {
			if let Some(name) = maybe_name {
				version_names.insert(name, version_key.clone());
			}

			let mut version = Version::new(&self.directory, &version_key);
			version.hydrate()?;
			versions.insert(version_key, version);
		}

		Ok(())
	}

	// check upstream and update version state
	async fn update(&self) -> Result<()> {
		// Ensure that the persistance folder exists before trying to write anything to it.
		fs::create_dir_all(&self.directory)?;

		let pending_repository_patches = self
			.repositories
			.iter()
			.map(|repository_name| self.update_repository_patches(repository_name));

		let repository_patches = try_join_all(pending_repository_patches).await?;

		// Get the latest patches for the updated patch lists.
		let latest_patches = repository_patches
			.iter()
			.zip(&self.repositories)
			.map(|(patches, repository_name)| {
				patches
					.last()
					.ok_or_else(|| anyhow!("no patches found for repository {repository_name}"))
			})
			.collect::<Result<Vec<_>>>()?;

		let key = version_key(&latest_patches);

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

		// Build the full version listing for persisting.
		let version_names = self.version_names.read().expect("poisoned");
		let name_lookup = version_names
			.iter()
			.map(|(name, version)| (version, name))
			.collect::<HashMap<_, _>>();
		let all_versions = versions
			.keys()
			.map(|version| (version, name_lookup.get(version)))
			.collect::<HashMap<_, _>>();

		let file = fs::File::options()
			.create(true)
			.write(true)
			.open(self.directory.join("versions.json"))?;
		file.lock_exclusive()?;
		file.set_len(0)?;

		serde_json::to_writer_pretty(file, &all_versions)?;

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

	// do i want a .flush? or is that handled as part of updates?
}

fn version_key(latest_patches: &[impl AsRef<str>]) -> String {
	let mut hasher = DefaultHasher::new();
	for patch_name in latest_patches {
		patch_name.as_ref().hash(&mut hasher);
	}
	let hash = hasher.finish();

	format!("{hash:X}")
}

#[derive(Debug)]
struct PatchStore {
	patches: HashMap<String, Patch>,

	path: PathBuf,
}

impl PatchStore {
	fn new(directory: &Path, repository_name: &str) -> Self {
		Self {
			patches: Default::default(),
			path: directory.join(format!("patches-{repository_name}.json")),
		}
	}

	fn hydrate(&mut self) -> Result<()> {
		// Try to open the file for this patch store - if it doesn't exist, it's probably not been saved before.
		let file = match fs::File::open(&self.path) {
			Ok(file) => file,
			Err(error) => match error.kind() {
				io::ErrorKind::NotFound => return Ok(()),
				_ => Err(error)?,
			},
		};
		file.lock_shared()?;

		let patches: Vec<Patch> = serde_json::from_reader(file)?;

		self.patches
			.extend(patches.into_iter().map(|patch| (patch.name.clone(), patch)));

		Ok(())
	}

	fn update(&mut self, patches: Vec<Patch>) -> Result<()> {
		// TODO: This currently just blindly updates everything and flushes everything to disk. Probably fine; but if this thrashes, can avoid writes if no changes found etc.
		self.patches
			.extend(patches.into_iter().map(|patch| (patch.name.clone(), patch)));

		// Open, lock, _then_ truncate, so we don't accidentally truncate an in-use file.
		let file = fs::File::options()
			.create(true)
			.write(true)
			.open(&self.path)?;
		file.lock_exclusive()?;
		file.set_len(0)?;

		serde_json::to_writer_pretty(file, &self.patches.values().collect::<Vec<_>>())?;

		Ok(())
	}
}

#[derive(Debug)]
struct Version {
	patches: HashMap<String, Vec<String>>,

	path: PathBuf,
}

impl Version {
	fn new(directory: &Path, key: &str) -> Self {
		Self {
			patches: Default::default(),
			path: directory.join(format!("version-{key}.json")),
		}
	}

	fn hydrate(&mut self) -> Result<()> {
		let file = match fs::File::open(&self.path) {
			Ok(file) => file,
			Err(error) => match error.kind() {
				io::ErrorKind::NotFound => return Ok(()),
				_ => Err(error)?,
			},
		};
		file.lock_shared()?;

		self.patches = serde_json::from_reader(file)?;

		Ok(())
	}

	fn update(&mut self, patches: HashMap<String, Vec<String>>) -> Result<()> {
		self.patches = patches;

		let file = fs::File::options()
			.create(true)
			.write(true)
			.open(&self.path)?;
		file.lock_exclusive()?;
		file.set_len(0)?;

		serde_json::to_writer_pretty(file, &self.patches)?;

		Ok(())
	}
}
