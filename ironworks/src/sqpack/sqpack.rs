use std::io::{Read, Seek};

pub trait Resource {
	fn path_metadata<'a>(&self, path: &'a str) -> Option<(&'a str, &'a str)>;

	type Index: Read + Seek;
	fn index(&self, repository: &str, category: &str, chunk: u8) -> Self::Index;

	type Index2: Read + Seek;
	fn index2(&self, repository: &str, category: &str, chunk: u8) -> Self::Index2;

	type Dat: Read + Seek;
	fn dat(&self, repository: &str, category: &str, chunk: u8) -> Self::Dat;
}

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

#[derive(Debug)]
pub struct SqPack<R> {
	resource: R,
}

impl<R: Resource> SqPack<R> {
	// default new should probably be in a sane resource default? or nah
	pub fn new(resource: R) -> Self {
		Self { resource }
	}

	// TODO: name
	pub fn read(&self, path: &str) {
		let foo = self.resource.path_metadata(path);
		println!("{foo:?}");
	}
}
