use std::{
	collections::{BTreeMap, HashMap},
	path::Path,
};

use anyhow::Result;

use super::persist::JsonFile;

#[derive(Debug)]
pub struct Version {
	patches: HashMap<String, Vec<String>>,
	file: JsonFile,
}

impl Version {
	pub fn new(directory: &Path, key: &str) -> Self {
		Self {
			patches: Default::default(),
			file: JsonFile::new(directory.join(format!("version-{key}.json"))),
		}
	}

	pub fn patches(&self) -> &HashMap<String, Vec<String>> {
		&self.patches
	}

	pub fn hydrate(&mut self) -> Result<()> {
		self.patches = self.file.read()?;
		Ok(())
	}

	pub fn update(&mut self, patches: HashMap<String, Vec<String>>) -> Result<()> {
		self.patches = patches;
		self.file
			.write(&self.patches.iter().collect::<BTreeMap<_, _>>())?;
		Ok(())
	}
}
