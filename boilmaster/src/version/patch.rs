use std::{collections::HashMap, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::persist::JsonFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
	pub name: String,
	pub url: String,
	pub size: u64,
}

#[derive(Debug)]
pub struct PatchStore {
	patches: HashMap<String, Patch>,

	file: JsonFile,
}

impl PatchStore {
	pub fn new(directory: &Path, repository_name: &str) -> Self {
		Self {
			patches: Default::default(),
			file: JsonFile::new(directory.join(format!("patches-{repository_name}.json"))),
		}
	}

	pub fn patch(&self, name: &str) -> Option<Patch> {
		self.patches.get(name).cloned()
	}

	pub fn hydrate(&mut self) -> Result<()> {
		let patches: Vec<Patch> = self.file.read()?;

		self.patches
			.extend(patches.into_iter().map(|patch| (patch.name.clone(), patch)));

		Ok(())
	}

	pub fn update(&mut self, patches: Vec<Patch>) -> Result<()> {
		// TODO: This currently just blindly updates everything and flushes everything to disk. Probably fine; but if this thrashes, can avoid writes if no changes found etc.
		self.patches
			.extend(patches.into_iter().map(|patch| (patch.name.clone(), patch)));

		let mut items = self.patches.iter().collect::<Vec<_>>();
		items.sort_by(|a, b| a.0.cmp(b.0));

		self.file
			.write(&items.into_iter().map(|a| a.1).collect::<Vec<_>>())?;

		Ok(())
	}
}
