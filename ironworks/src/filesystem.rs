use std::io;

pub trait Filesystem {
	type File;
	type Error: std::error::Error + 'static;

	fn file(&self, path: &str) -> Result<Self::File, Self::Error>;
}

pub trait Version {
	type Error: std::error::Error + 'static;

	fn version(&self) -> Result<String, Self::Error>;
}
