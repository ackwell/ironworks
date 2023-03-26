use std::sync::Arc;

use ironworks::{
	excel::{Excel, Language},
	sqpack::SqPack,
	Ironworks,
};
use serde::Deserialize;

use crate::version::PatchList;

use super::{
	language::LanguageString,
	patch::{self, wip_build_zipatch_view},
};

#[derive(Debug, Deserialize)]
pub struct Config {
	patch: patch::Config,

	language: LanguageString,
}

pub struct Data {
	default_language: Language,

	// TODO: this should be a lazy map of some kind once this is using real data
	// TODO: might want to be eagerly constructed, with lazy excel internal construction, given it will likely need to download patches. building excel isn't expensive so might not need to be lazy on that front.
	temp_version: Version,
}

impl Data {
	pub fn new(config: Config, temp_patch_list: PatchList) -> Self {
		Data {
			default_language: config.language.into(),
			temp_version: Version::new(config.patch, temp_patch_list),
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
	fn new(temp_config: patch::Config, temp_patch_list: PatchList) -> Self {
		/*
		BIG TODO POINT:
		at the moment this flow is safe because we only ever work with a single version. once there's two versions in the mix, this has an opportunity to end up with two builders checking for a patch at the same time as a race and downloading the file twice. fix will likely require _some_ form of consolidated coordination of "hey yeah this is being downloaded already" - will be able to use the semaphore ctor point for this purpose i assume?
		i'm guessing that, to handle that, i'll need a patch manager that's shared between the data instances akin to a few other systems around the place.
		*/

		// TODO: This is horrible and needs to be removed. Don't merge this with this here.
		let view =
			futures::executor::block_on(wip_build_zipatch_view(temp_config, temp_patch_list))
				.expect("TODO");

		let ironworks = Ironworks::new().with_resource(SqPack::new(view));
		let excel = Excel::with().build(Arc::new(ironworks));

		Version {
			excel: Arc::new(excel),
		}
	}

	pub fn excel(&self) -> Arc<Excel<'static>> {
		Arc::clone(&self.excel)
	}
}
