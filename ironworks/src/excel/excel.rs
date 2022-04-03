use super::resource::Resource;

/// An excel database.
#[derive(Debug)]
pub struct Excel<R> {
	resource: R,
}

impl<R: Resource> Excel<R> {
	/// Build a representation of an Excel database.
	pub fn new(resource: R) -> Self {
		Self { resource }
	}
}
