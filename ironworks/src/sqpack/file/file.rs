use std::{io, sync::Arc};

use binrw::BinRead;

use crate::{
	filesystem::Version,
	sqpack::{
		Resource,
		block::BlockStream,
		error::{Error, Result},
		sqpack::Location,
	},
};

use super::{
	empty, model,
	shared::{FileKind, Header},
	standard, texture,
};

// Wrapper struct to prevent the innards of the file streams from being public API surface.
/// A stream of data for a file read from a sqpack dat archive.
#[derive(Debug)]
pub struct File<R: Resource> {
	resource: Arc<R>,
	location: Location,

	reader: Option<FileReader<R::File>>,
}

#[derive(Debug)]
enum FileReader<R> {
	Empty(io::Empty),
	Standard(BlockStream<R>),
	Model(io::Cursor<Vec<u8>>),
	Texture(io::Cursor<Vec<u8>>),
}

impl<R: Resource> File<R> {
	pub(crate) fn new(resource: Arc<R>, location: Location) -> Self {
		Self {
			resource,
			location,
			reader: None,
		}
	}

	fn reader(&mut self) -> &mut FileReader<R::File> {
		if self.reader.is_none() {
			self.reader = Some(self.build_reader())
		}

		self.reader.as_mut().expect("reader should not be None")
	}

	fn build_reader(&self) -> FileReader<R::File> {
		let mut reader = self
			.resource
			.file(
				self.location.repository,
				self.location.category,
				self.location.index.clone(),
			)
			.expect("TODO");

		let header = Header::read(&mut reader).expect("TODO");

		use FileReader as FSK;
		match &header.kind {
			FileKind::Empty => FSK::Empty(empty::read(reader, header).expect("TODO")),
			FileKind::Standard => {
				FSK::Standard(standard::read(reader, header.size, header).expect("TODO"))
			}
			FileKind::Model => FSK::Model(model::read(reader, header.size, header).expect("TODO")),
			FileKind::Texture => {
				FSK::Texture(texture::read(reader, header.size, header).expect("TODO"))
			}
		}
	}
}

impl<R: Resource> Version for File<R> {
	type Error = Error;

	fn version(&self) -> std::result::Result<String, Self::Error> {
		self.resource.version(self.location.repository)
	}
}

impl<R: Resource> io::Read for File<R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		use FileReader as FR;
		match self.reader() {
			FR::Empty(reader) => reader.read(buf),
			FR::Standard(reader) => reader.read(buf),
			FR::Model(reader) => reader.read(buf),
			FR::Texture(reader) => reader.read(buf),
		}
	}
}
