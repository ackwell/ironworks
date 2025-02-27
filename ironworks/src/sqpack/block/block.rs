use std::io::{self, Read, Seek, SeekFrom, Take};

use binrw::{BinRead, binread};
use either::Either;
use flate2::read::DeflateDecoder;

const MAX_COMPRESSED_BLOCK_SIZE: u32 = 16_000;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct BlockHeader {
	pub size: u32,
	// unknown1: u32,
	#[br(pad_before = 4)]
	pub compressed_size: u32,
	pub decompressed_size: u32,
}

pub fn read_block<R: Read + Seek>(reader: &mut R, offset: u32) -> io::Result<BlockPayload<R>> {
	// Seek to the block and read its header so we know how much to expect in the rest of the block.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let block_header =
		BlockHeader::read(reader).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

	Ok(BlockPayload::new(
		reader,
		block_header.compressed_size,
		block_header.decompressed_size,
	))
}

/// Reader for a single potentially-compressed block payload.
#[derive(Debug)]
pub struct BlockPayload<'a, R> {
	block_reader: Either<Take<&'a mut R>, DeflateDecoder<Take<&'a mut R>>>,
}

impl<'a, R: Read + Seek> BlockPayload<'a, R> {
	/// Construct a new block payload reader, reading from the provided reader at
	/// it's current position. The reader will be advanced to the end of the payload.
	pub fn new(reader: &'a mut R, input_size: u32, output_size: u32) -> Self {
		// TODO: Look into the padding on compressed blocks, there's some funky stuff going on in some cases. Ref. Coinach/IO/File & Lumina.

		let block_reader = match input_size > MAX_COMPRESSED_BLOCK_SIZE {
			true => Either::Left(reader.take(output_size.into())),
			false => Either::Right(DeflateDecoder::new(reader.take(input_size.into()))),
		};

		Self { block_reader }
	}
}

impl<R: Read> Read for BlockPayload<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.block_reader.read(buf)
	}
}
