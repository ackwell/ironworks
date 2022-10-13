use std::io::{Cursor, Empty, Read, Seek, SeekFrom};

use binrw::BinRead;

use crate::error::Result;

use super::{
	empty, model,
	shared::{FileKind, Header},
	standard, texture,
};

pub fn read<R: Read + Seek>(mut reader: R, offset: u32) -> Result<FileStream<R>> {
	// Move to the start of the file and read in the header.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let header = Header::read(&mut reader)?;

	let file_offset = offset + header.size;

	use FileStreamKind as FSK;
	let file_stream = match &header.kind {
		FileKind::Empty => FSK::Empty(empty::read(reader, header)?),
		FileKind::Standard => FSK::Standard(standard::read(reader, file_offset, header)?),
		FileKind::Model => FSK::Model(model::read(reader, file_offset, header)?),
		FileKind::Texture => FSK::Texture(texture::read(reader, file_offset, header)?),
	};

	Ok(FileStream { inner: file_stream })
}

// Wrapper struct to prevent the innards of the file streams from being public API surface.
/// A stream of data for a file read from a sqpack dat archive.
#[derive(Debug)]
pub struct FileStream<R> {
	inner: FileStreamKind<R>,
}

#[derive(Debug)]
enum FileStreamKind<R> {
	Empty(Empty),
	Standard(standard::FileStream<R>),
	Model(Cursor<Vec<u8>>),
	Texture(Cursor<Vec<u8>>),
}

impl<R: Read + Seek> Read for FileStream<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		use FileStreamKind as FSK;
		match &mut self.inner {
			FSK::Empty(stream) => stream.read(buf),
			FSK::Standard(stream) => stream.read(buf),
			FSK::Model(stream) => stream.read(buf),
			FSK::Texture(stream) => stream.read(buf),
		}
	}
}

impl<R: Read + Seek> Seek for FileStream<R> {
	fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
		use FileStreamKind as FSK;
		match &mut self.inner {
			FSK::Empty(stream) => stream.seek(pos),
			FSK::Standard(stream) => stream.seek(pos),
			FSK::Model(stream) => stream.seek(pos),
			FSK::Texture(stream) => stream.seek(pos),
		}
	}
}
