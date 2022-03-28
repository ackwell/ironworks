use super::resource::Resource;

// should there be a xiv feature?
pub struct FfxivFsResource {}

impl Resource for FfxivFsResource {
	fn path_metadata<'a>(&self, path: &'a str) -> Option<(&'a str, &'a str)> {
		let split = path.split('/').take(2).collect::<Vec<_>>();
		// TODO: Handle defaulting the repo
		match split[..] {
			[category, repository] => Some((repository, category)),
			_ => None,
		}
	}

	type Index = std::io::Empty;
	fn index(&self, _repository: &str, _category: &str, _chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Index2 = std::io::Empty;
	fn index2(&self, _repository: &str, _category: &str, _chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Dat = std::io::Empty;
	fn dat(&self, _repository: &str, _category: &str, _chunk: u8) -> Self::Index {
		std::io::empty()
	}
}
