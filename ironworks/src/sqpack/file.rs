use std::io::{self, Cursor, Read, Seek, SeekFrom};

use binrw::{binread, BinRead};
use flate2::read::DeflateDecoder;

use crate::error::{Error, Result};

const MAX_COMPRESSED_BLOCK_SIZE: u32 = 16_000;

pub fn read_file(mut reader: impl Read + Seek, offset: u32) -> Result<Vec<u8>> {
	// Move to the start of the file and read in the header.
	reader
		.seek(SeekFrom::Start(offset.into()))
		.map_err(|error| Error::Resource(error.into()))?;
	let header = Header::read(&mut reader).map_err(|error| Error::Resource(error.into()))?;

	// TODO: Check the raw file size?

	match &header.kind {
		FileKind::Standard => read_standard(reader, offset, header),
		_ => todo!("File kind: {:?}", header.kind),
	}
}

fn read_standard(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
	// Read each block into a final byte vector.
	let out_buffer = header
		.blocks
		.iter()
		.try_fold(
			Vec::<u8>::new(),
			|mut vec, block_info| -> io::Result<Vec<u8>> {
				let mut block_reader = read_block(&mut reader, block_info, offset + header.size)?;
				block_reader.read_to_end(&mut vec)?;
				Ok(vec)
			},
		)
		.map_err(|error| Error::Resource(error.into()))?;

	Ok(out_buffer)
}

// TODO: move this into a block struct of some kind if we do lazy reading?
fn read_block(
	reader: &mut (impl Read + Seek),
	block_info: &BlockInfo,
	base: u32,
) -> io::Result<BlockReader> {
	// Read the block into memory
	let mut buffer = vec![0u8; block_info.compressed_size.into()];
	reader.seek(SeekFrom::Start((base + block_info.offset).into()))?;
	reader.read_exact(&mut buffer)?;
	let mut raw_cursor = Cursor::new(buffer);

	// TODO: if type 1 and first 64 == second 64, RSF
	//       if type 1 and first 64 == [0..], empty

	// Read out the inline block header
	let block_header = BlockHeader::read(&mut raw_cursor)
		.map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

	// TODO: Should probably be an Error::Resource
	assert_eq!(
		block_header.decompressed_size,
		block_info.decompressed_size.into(),
		"Block info and header decompressed size differs."
	);

	// TODO: Look into the padding on compressed blocks, there's some funky stuff going on in some cases. Ref. Coinach/IO/File & Lumina.

	// Build a block reader for this block
	let block_reader = if block_header.decompressed_size > MAX_COMPRESSED_BLOCK_SIZE {
		BlockReader::Loose(raw_cursor)
	} else {
		BlockReader::Compressed(DeflateDecoder::new(raw_cursor))
	};

	Ok(block_reader)
}

enum BlockReader {
	Loose(Cursor<Vec<u8>>),
	Compressed(DeflateDecoder<Cursor<Vec<u8>>>),
}

impl Read for BlockReader {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::Loose(reader) => reader.read(buf),
			Self::Compressed(reader) => reader.read(buf),
		}
	}
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct Header {
	size: u32,
	kind: FileKind,
	_raw_file_size: u32,
	#[br(temp, pad_before = 8)]
	block_count: u32,
	#[br(count = block_count)]
	blocks: Vec<BlockInfo>,
}

#[binread]
#[derive(Debug)]
#[br(little, repr = u32)]
enum FileKind {
	Empty = 1,
	Standard,
	Model,
	Texture,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockInfo {
	offset: u32,
	compressed_size: u16,
	decompressed_size: u16,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockHeader {
	_size: u32,
	// unknown1: u32,
	#[br(pad_before = 4)]
	_compressed_size: u32,
	decompressed_size: u32,
}
