use std::{
	fmt::Display,
	io::{self, Read, Seek, SeekFrom},
};

use binrw::{binread, BinRead};
use flate2::read::DeflateDecoder;

const MAX_COMPRESSED_BLOCK_SIZE: u32 = 16_000;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Header {
	pub size: u32,
	pub kind: FileKind,
	pub raw_file_size: u32,
	// num_blocks: u32,
	// block_buffer_size: u32,
	#[br(pad_before = 8)]
	pub block_count: u32,
}

#[binread]
#[derive(Debug)]
#[br(little, repr = u32)]
pub enum FileKind {
	Empty = 1,
	Standard,
	Model,
	Texture,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockHeader {
	_size: u32,
	// unknown1: u32,
	#[br(pad_before = 4)]
	compressed_size: u32,
	decompressed_size: u32,
}

pub fn read_failed(item: impl Display, expected: impl Display, got: impl Display) -> String {
	format!("Failed to read {item}. Expected {expected} bytes, got {got}.",)
}

// TODO: move this into a block struct of some kind if we do lazy reading?
pub fn read_block<R: Read + Seek>(reader: &mut R, offset: u32) -> io::Result<BlockReader<R>> {
	// Seek to the block and read its header so we know how much to expect in the rest of the block.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let block_header =
		BlockHeader::read(reader).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

	// TODO: Look into the padding on compressed blocks, there's some funky stuff going on in some cases. Ref. Coinach/IO/File & Lumina.

	// Build a reader for the block.
	let reader = match block_header.compressed_size > MAX_COMPRESSED_BLOCK_SIZE {
		true => BlockReader::Loose(reader.take(block_header.decompressed_size.into())),
		false => BlockReader::Compressed(DeflateDecoder::new(
			reader.take(block_header.compressed_size.into()),
		)),
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
