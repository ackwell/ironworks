use std::io::{self, Read, Seek, SeekFrom};

use flate2::read::DeflateDecoder;

use crate::file::patch::BlockHeader;

// TODO: This is pretty much a copy paste from sqpack::file::shared - work out how this can be reused
pub fn read_block<'a, R: Read + Seek>(
	reader: &'a mut R,
	header: &BlockHeader,
) -> io::Result<BlockReader<'a, R>> {
	// Seek to the block and read its header so we know how much to expect in the rest of the block.
	// reader.seek(SeekFrom::Start(offset.into()))?;
	// let block_header =
	// 	BlockHeader::read(reader).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
	reader.seek(SeekFrom::Start(header.offset()))?;

	// TODO: Look into the padding on compressed blocks, there's some funky stuff going on in some cases. Ref. Coinach/IO/File & Lumina.

	// Build a reader for the block.
	let reader = match header.is_compressed() {
		true => BlockReader::Compressed(DeflateDecoder::new(
			reader.take(header.compressed_size().into()),
		)),
		false => BlockReader::Loose(reader.take(header.decompressed_size().into())),
	};

	Ok(reader)
}

pub enum BlockReader<'a, R> {
	Loose(io::Take<&'a mut R>),
	Compressed(DeflateDecoder<io::Take<&'a mut R>>),
}

impl<R: Read> Read for BlockReader<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::Loose(reader) => reader.read(buf),
			Self::Compressed(reader) => reader.read(buf),
		}
	}
}
