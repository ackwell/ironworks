use std::{fs, io, path::PathBuf};

use anyhow::Result;
use fs4::FileExt;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct JsonFile {
	path: PathBuf,
}

impl JsonFile {
	pub fn new(path: PathBuf) -> Self {
		Self { path }
	}

	pub fn read<T>(&self) -> Result<T>
	where
		T: for<'de> Deserialize<'de>,
		T: Default,
	{
		let file = match fs::File::open(&self.path) {
			Ok(file) => file,
			Err(error) => {
				return match error.kind() {
					io::ErrorKind::NotFound => Ok(T::default()),
					_ => Err(error.into()),
				}
			}
		};

		file.lock_shared()?;

		let value = serde_json::from_reader(file)?;

		Ok(value)
	}

	pub fn write<T>(&self, value: &T) -> Result<()>
	where
		T: Serialize,
	{
		// Open, lock, _then_ truncate, so we don't accidentally truncate an in-use file.
		let file = fs::File::options()
			.create(true)
			.write(true)
			.open(&self.path)?;
		file.lock_exclusive()?;
		file.set_len(0)?;

		serde_json::to_writer_pretty(file, value)?;

		Ok(())
	}
}
