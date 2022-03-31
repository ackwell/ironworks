use std::io::{self, Cursor, Read, Seek, SeekFrom};

use binrw::{binread, BinRead};
use flate2::read::DeflateDecoder;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct File<R> {
	reader: R,
	header: Header,
	base: u32,
	cursor: Option<Cursor<Vec<u8>>>,
}

impl<R> File<R>
where
	R: Read + Seek,
{
	pub fn new(mut reader: R, offset: u32) -> Result<Self> {
		// todo make reading the file header lazy too?
		reader
			.seek(SeekFrom::Start(offset.into()))
			.map_err(|_| Error::Resource)?;
		let header = Header::read(&mut reader).map_err(|_| Error::Resource)?;

		Ok(Self {
			reader,
			base: offset + header.size,
			header,
			cursor: None,
		})
	}

	fn cursor(&mut self) -> io::Result<&mut Cursor<Vec<u8>>> {
		// Check if we have a cached cursor - if we do, we can exit early with it
		let cursor_cache = &mut self.cursor;
		if let Some(cursor) = cursor_cache {
			return Ok(cursor);
		}

		// Read each block into a final byte vector
		let out_buffer = self.header.blocks.iter().try_fold(
			Vec::<u8>::new(),
			|mut vec, block_info| -> io::Result<Vec<u8>> {
				let mut block_reader = read_block(&mut self.reader, block_info, self.base)?;
				block_reader.read_to_end(&mut vec)?;
				Ok(vec)
			},
		)?;

		Ok(cursor_cache.insert(Cursor::new(out_buffer)))
	}
}

impl<R: Read + Seek> Read for File<R> {
	// TODO: Look into making this lazier, i.e. per-block lazy or similar
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		self.cursor()?.read(buffer)
	}
}

// TODO: move this into a block struct of some kind if we do lazy reading?
fn read_block<R>(reader: &mut R, block_info: &BlockInfo, base: u32) -> io::Result<BlockReader>
where
	R: Read + Seek,
{
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

	// todo assert uncompressed is equal
	// compressed isn't equal, check coinach/io/file there's some funky padding

	// Build a block reader for this block
	// todo where put constant!?!?
	let block_reader = if block_header.decompressed_size > 16000 {
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
	raw_file_size: u32,
	#[br(pad_before = 8)]
	block_count: u32,
	#[br(count = block_count)]
	blocks: Vec<BlockInfo>,
}

// TODO name
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
	size: u32,
	// unknown1: u32,
	#[br(pad_before = 4)]
	compressed_size: u32,
	decompressed_size: u32,
}
