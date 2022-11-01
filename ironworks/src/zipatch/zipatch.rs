use super::{cache::PatchCache, version::Version};

#[derive(Debug)]
pub struct ZiPatch {
	// TODO: this should probably be arc'd. Interior or exterior mutability?
	cache: PatchCache,
}

impl ZiPatch {
	// TODO: API. This should probably take an initial list of patches, grouped by repository or similar, and ordered by their requisite application/dependency order.. Consider if new() should care about how a version is defined, or if that's a .version() problem - i'm leaning to the latter.
	pub fn new() -> Self {
		Self {
			cache: PatchCache::new(),
		}
	}

	// TODO: API. Assuming going with the latter from new()'s comment, this should accept some "version" concept that declares the patch point for each repository.
	pub fn version(&self) -> Version {
		Version::new()
	}
}
