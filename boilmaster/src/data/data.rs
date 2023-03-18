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
			temp_version: Version::new(temp_view),
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
	fn new(temp_view: zipatch::View) -> Self {
		let ironworks = Ironworks::new().with_resource(SqPack::new(temp_view));
		let excel = Excel::with().build(Arc::new(ironworks));

		Version {
			excel: Arc::new(excel),
		}
	}

	pub fn excel(&self) -> Arc<Excel<'static>> {
		Arc::clone(&self.excel)
	}
}
