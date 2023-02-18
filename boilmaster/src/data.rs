use std::sync::Arc;

use ironworks::{
	excel::{Excel, Language},
	sqpack::SqPack,
	zipatch::{VersionSpecifier, ZiPatch},
	Ironworks,
};

pub struct Data {
	// TODO: this should be a lazy map of some kind once this is using real data
	temp_version: Version,
}

impl Data {
	pub fn new(temp_zipatch: ZiPatch) -> Self {
		Data {
			temp_version: Version::new(temp_zipatch),
		}
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
	fn new(temp_zipatch: ZiPatch) -> Self {
		// TODO: Given how different versions will end up having different paths through the patch files, i'm tempted to say that zipatch having versioning in itself is useless and should be removed.
		let zipatch_version = temp_zipatch.version(VersionSpecifier::latest());

		// TODO: Work out how to handle languages
		// let ironworks = Ironworks::new().with_resource(SqPack::new(Install::search().unwrap()));
		let ironworks = Ironworks::new().with_resource(SqPack::new(zipatch_version));
		let excel = Excel::with()
			.language(Language::English)
			.build(Arc::new(ironworks));

		Version {
			excel: Arc::new(excel),
		}
	}

	pub fn excel(&self) -> Arc<Excel<'static>> {
		Arc::clone(&self.excel)
	}
}
