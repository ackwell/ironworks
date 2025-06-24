use std::{io, sync::Arc};

use crate::{filesystem::Version, sqpack::Resource};

use super::{
	error::{Error, Result},
	format::Format,
	index::Location,
};

#[derive(Debug)]
pub struct File<R: Resource> {
	resource: Arc<R>,

	repository: u8,
	category: u8,
	location: Location,

	format: Option<Format<R::File>>,
}

impl<R> File<R>
where
	R: Resource,
{
	pub(super) fn new(resource: Arc<R>, repository: u8, category: u8, location: Location) -> Self {
		Self {
			resource,
			repository,
			category,
			location,
			format: None,
		}
	}

	fn format(&mut self) -> Result<&mut Format<R::File>> {
		if self.format.is_none() {
			let format = Format::new(self.resource.file(
				self.repository,
				self.category,
				self.location.clone(),
			)?)?;
			self.format = Some(format);
		}

		Ok(self.format.as_mut().expect("format should not be None"))
	}
}

impl<R> Version for File<R>
where
	R: Resource,
{
	type Error = Error;

	fn version(&self) -> std::result::Result<String, Self::Error> {
		self.resource.version(self.repository)
	}
}

impl<R> io::Read for File<R>
where
	R: Resource,
{
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.format().map_err(into_io_error)?.read(buf)
	}
}

impl<R> io::Seek for File<R>
where
	R: Resource,
{
	fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
		self.format().map_err(into_io_error)?.seek(pos)
	}
}

fn into_io_error(error: Error) -> io::Error {
	let kind = match error {
		Error::Io(error) => return error,
		Error::FileNotFound => io::ErrorKind::NotFound,
		Error::PathInvalid(_) => io::ErrorKind::Other,
		Error::FileIncomplete(_) => io::ErrorKind::Other,
		Error::Malformed(_) => io::ErrorKind::InvalidData,
	};
	io::Error::new(kind, error)
}
