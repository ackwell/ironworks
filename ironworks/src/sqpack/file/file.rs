use std::io::{Cursor, Empty, Read, Seek, SeekFrom};

use binrw::BinRead;

use crate::error::{Error, Result};

use super::{
	empty, model,
	shared::{read_failed, FileKind, Header},
	standard, texture,
};

pub fn read<R: Read + Seek>(mut reader: R, offset: u32) -> Result<Vec<u8>> {
	// Move to the start of the file and read in the header.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let header = Header::read(&mut reader)?;

	let expected_file_size = header.raw_file_size;

	let file_offset = offset + header.size;
	let mut file_stream = match &header.kind {
		FileKind::Empty => FileStream::Empty(empty::read(reader, header)?),
		FileKind::Standard => FileStream::Standard(standard::read(reader, file_offset, header)?),
		FileKind::Model => FileStream::Model(model::read(reader, file_offset, header)?),
		FileKind::Texture => FileStream::Texture(texture::read(reader, file_offset, header)?),
	};

	let mut out_buffer = Vec::with_capacity(expected_file_size.try_into().unwrap());
	file_stream.read_to_end(&mut out_buffer)?;

	match out_buffer.len() == expected_file_size.try_into().unwrap() {
		true => Ok(out_buffer),
		false => Err(Error::Resource(
			read_failed("file", expected_file_size, out_buffer.len()).into(),
		)),
	}
}

enum FileStream<R> {
	Empty(Empty),
	Standard(standard::FileStream<R>),
	Model(Cursor<Vec<u8>>),
	Texture(Cursor<Vec<u8>>),
}

impl<R: Read + Seek> Read for FileStream<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		match self {
			Self::Empty(stream) => stream.read(buf),
			Self::Standard(stream) => stream.read(buf),
			Self::Model(stream) => stream.read(buf),
			Self::Texture(stream) => stream.read(buf),
		}
	}
}
