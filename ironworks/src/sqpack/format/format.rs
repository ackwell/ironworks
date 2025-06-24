use std::{io, sync::Arc};

use binrw::BinRead;

use crate::{
	filesystem::Version,
	sqpack::{
		Resource,
		block::BlockStream,
		error::{Error, Result},
	},
};

use super::{
	empty, model,
	shared::{FileKind, Header},
	standard, texture,
};

#[derive(Debug)]
pub enum Format<R> {
	Empty(io::Empty),
	Standard(BlockStream<R>),
	Model(io::Cursor<Vec<u8>>),
	Texture(io::Cursor<Vec<u8>>),
}

impl<R> Format<R>
where
	R: io::Read + io::Seek,
{
	pub fn new(mut reader: R) -> Result<Self> {
		let header = Header::read(&mut reader)?;

		use Format as F;
		let format = match &header.kind {
			FileKind::Empty => F::Empty(empty::read(reader, header)?),
			FileKind::Standard => F::Standard(standard::read(reader, header.size, header)?),
			FileKind::Model => F::Model(model::read(reader, header.size, header)?),
			FileKind::Texture => F::Texture(texture::read(reader, header.size, header)?),
		};

		Ok(format)
	}
}

impl<R> io::Read for Format<R>
where
	R: io::Read + io::Seek,
{
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::Empty(reader) => reader.read(buf),
			Self::Standard(reader) => reader.read(buf),
			Self::Model(reader) => reader.read(buf),
			Self::Texture(reader) => reader.read(buf),
		}
	}
}

impl<R> io::Seek for Format<R>
where
	R: io::Read + io::Seek,
{
	fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
		match self {
			Self::Empty(reader) => reader.seek(pos),
			Self::Standard(reader) => reader.seek(pos),
			Self::Model(reader) => reader.seek(pos),
			Self::Texture(reader) => reader.seek(pos),
		}
	}
}
