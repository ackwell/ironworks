use std::{
	convert::Infallible,
	fmt::Debug,
	sync::{Arc, OnceLock},
};

use derivative::Derivative;

use crate::{
	file::exl,
	filesystem::{Filesystem, Version},
	utility::{HashMapCache, HashMapCacheExt},
};

use super::{
	error::{Error, Result},
	language::Language,
	path,
};

/// An Excel database.
#[derive(Debug)]
pub struct Excel<F> {
	filesystem: F,

	default_language: Language,
}

impl<F> Excel<F> {
	/// Build an view into the Excel database for a given filesystem.
	pub fn new(filesystem: F) -> Self {
		Self {
			filesystem,

			default_language: Language::None,
		}
	}

	/// Set the default language to use when reading from the database.
	pub fn with_default_language(mut self, language: Language) -> Self {
		self.set_default_language(language);
		self
	}

	/// Set the default language to use when reading from the database.
	pub fn set_default_language(&mut self, language: Language) {
		self.default_language = language;
	}
}

impl<F> Excel<F>
where
	F: Filesystem,
	F::File: Version,
{
	pub fn version(&self) -> Result<String> {
		let file = self.filesystem.file(path::exl()).map_err(fs_err)?;
		let version = file.version().map_err(fs_err)?;

		Ok(version)
	}
}

fn fs_err(error: impl std::error::Error + 'static) -> Error {
	Error::Filesystem(error.into())
}
