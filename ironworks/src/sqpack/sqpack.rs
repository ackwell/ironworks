use super::resource::Resource;

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
