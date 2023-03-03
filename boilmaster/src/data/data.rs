use std::sync::Arc;

use ironworks::{
	excel::{Excel, Language},
	sqpack::SqPack,
	zipatch::{VersionSpecifier, ZiPatch},
	Ironworks,
};
use serde::Deserialize;

use super::language::LanguageString;

#[derive(Debug, Deserialize)]
pub struct Config {
	language: LanguageString,
}

pub struct Data {
	default_language: Language,

	// TODO: this should be a lazy map of some kind once this is using real data
	temp_version: Version,
}

impl Data {
	pub fn new(config: Config, temp_zipatch: ZiPatch) -> Self {
		Data {
			default_language: config.language.into(),
			temp_version: Version::new(temp_zipatch, config.language.into()),
		}
	}

	pub fn default_language(&self) -> Language {
		self.default_language
	}

	pub fn version(&self, version: Option<&str>) -> &Version {
		// TODO: actual version handling, pulling data from an actual game install.
		if version.is_some() {
			todo!("data version handling");
		}

		&self.temp_version
	}
}

pub struct Version {
	excel: Arc<Excel<'static>>,
}

impl Version {
	fn new(temp_zipatch: ZiPatch, temp_language: Language) -> Self {
		// TODO: Given how different versions will end up having different paths through the patch files, i'm tempted to say that zipatch having versioning in itself is useless and should be removed.
		let zipatch_version = temp_zipatch.version(VersionSpecifier::latest());

		// TODO: temp language - ideally the root excel can be defaultless, and consumers can be explicit with their language usage. i'm making read explicit - this will mostly be a search thing. don't merge until done i guess.
		let ironworks = Ironworks::new().with_resource(SqPack::new(zipatch_version));
		let excel = Excel::with()
			.language(temp_language)
			.build(Arc::new(ironworks));

		Version {
			excel: Arc::new(excel),
		}
	}

	pub fn excel(&self) -> Arc<Excel<'static>> {
		Arc::clone(&self.excel)
	}
}
