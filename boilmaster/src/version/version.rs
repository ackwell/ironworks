use std::{
	collections::{BTreeMap, HashMap},
	fs, io,
	path::{Path, PathBuf},
	sync::RwLock,
};

use super::thaliak;
use anyhow::{anyhow, Context, Result};
use figment::value::magic::RelativePathBuf;
use fs4::FileExt;
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct Manager {
	thaliak: thaliak::Provider,
	directory: PathBuf,

	repositories: Vec<String>,
	patches: HashMap<String, RwLock<PatchStore>>,
	versions: RwLock<HashMap<String, Version>>,
	version_names: RwLock<HashMap<String, String>>,
}

impl Manager {
	pub fn new(config: Config) -> Result<Self> {
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

		let manager = Self {
			thaliak: thaliak::Provider::new(config.thaliak),
			directory,
			repositories: config.repositories,
			patches,
			versions: Default::default(),
			version_names: Default::default(),
		};

		manager.hydrate()?;

		Ok(manager)
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

	pub fn resolve(&self, name: &str) -> Option<String> {
		self.version_names
			.read()
			.expect("poisoned")
			.get(name)
			.cloned()
	}

	pub fn patch_list(&self, key: &str) -> Result<PatchList> {
		let versions = self.versions.read().expect("poisoned");
		let version = versions
			.get(key)
			.with_context(|| format!("unknown version {key}"))?;

		// TODO: A version made on repository list [a, b] will create a patch list [a] on an updated repository list of [a, X, b]. I'm not convinced that's a problem.
		let patch_list = self
			.repositories
			.iter()
			.map_while(|repository_name| {
				let patches = self.patches.get(repository_name)?.read().expect("poisoned");
				let patch_names = version.patches.get(repository_name)?;

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

	// check upstream and update version state
	pub async fn update(&self) -> Result<()> {
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

		// TEMP: for now, setting the __NONE sigil to always point to the most recent version key. Don't merge this, hey?
		let mut vn_temp = self.version_names.write().expect("poisoned");
		vn_temp.insert("__NONE".to_string(), key);
		drop(vn_temp);

		// Build the full version listing for persisting.
		let version_names = self.version_names.read().expect("poisoned");
		let name_lookup = version_names
			.iter()
			.map(|(name, version)| (version, name))
			.collect::<HashMap<_, _>>();
		let all_versions = versions
			.keys()
			.map(|version| (version, name_lookup.get(version)))
			.collect::<BTreeMap<_, _>>();

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
	let bytes = latest_patches
		.iter()
		.flat_map(|v| v.as_ref().as_bytes())
		.copied()
		.collect::<Vec<_>>();
	let hash = murmurhash32::murmurhash3(&bytes);

	format!("{hash:x}")
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

	fn patch(&self, name: &str) -> Option<Patch> {
		self.patches.get(name).cloned()
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

		let mut items = self.patches.iter().collect::<Vec<_>>();
		items.sort_by(|a, b| a.0.cmp(b.0));
		serde_json::to_writer_pretty(file, &items.into_iter().map(|a| a.1).collect::<Vec<_>>())?;

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

		serde_json::to_writer_pretty(file, &self.patches.iter().collect::<BTreeMap<_, _>>())?;

		Ok(())
	}
}
