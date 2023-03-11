use std::sync::Arc;

use ironworks::{
	excel::{Excel, Language},
	sqpack::SqPack,
	zipatch, Ironworks,
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
	pub fn new(config: Config, temp_view: zipatch::View) -> Self {
		Data {
			default_language: config.language.into(),
			temp_version: Version::new(temp_view, config.language.into()),
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
	fn new(temp_view: zipatch::View, temp_language: Language) -> Self {
		// TODO: temp language - ideally the root excel can be defaultless, and consumers can be explicit with their language usage. i'm making read explicit - this will mostly be a search thing. don't merge until done i guess.
		let ironworks = Ironworks::new().with_resource(SqPack::new(temp_view));
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
