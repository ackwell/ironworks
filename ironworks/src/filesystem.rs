use std::io;

use crate::file::{FromReader, ReadError};

pub trait Filesystem {
	type File;
	type Error: std::error::Error + 'static;

	fn file(&self, path: &str) -> Result<Self::File, Self::Error>;
}

pub trait FilesystemRead {
	type Error;

	fn read<T>(&self, path: &str) -> Result<T, Self::Error>
	where
		T: FromReader,
		Self::Error: From<ReadError>;
}

impl<F> FilesystemRead for F
where
	F: Filesystem,
	F::File: io::Read + io::Seek,
{
	type Error = F::Error;

	fn read<T>(&self, path: &str) -> Result<T, Self::Error>
	where
		T: FromReader,
		Self::Error: From<ReadError>,
	{
		Ok(T::read(self.file(path)?)?)
	}
}

pub trait Version {
	type Error: std::error::Error + 'static;

	fn version(&self) -> Result<String, Self::Error>;
}
