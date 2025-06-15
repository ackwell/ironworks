use std::{
	io::{Cursor, Empty, Read, Seek, SeekFrom},
	sync::Arc,
};

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
pub struct File<R> {
	resource: Arc<R>,
	location: Location,
}

#[derive(Debug)]
enum FileStreamKind<R> {
	Empty(Empty),
	Standard(BlockStream<R>),
	Model(Cursor<Vec<u8>>),
	Texture(Cursor<Vec<u8>>),
}

impl<R> File<R> {
	pub(crate) fn new(resource: Arc<R>, location: Location) -> Self {
		Self { resource, location }
	}
}

impl<R: Resource> Version for File<R> {
	type Error = Error;

	fn version(&self) -> std::result::Result<String, Self::Error> {
		self.resource.version(self.location.repository)
	}
}
