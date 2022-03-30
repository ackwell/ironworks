use std::io::{Read, Seek, SeekFrom};

use binrw::{binread, BinRead};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct File<R> {
	reader: R,
	header: Header,
	block_base: u32,
}

impl<R> File<R>
where
	R: Read + Seek,
{
	pub fn new(mut reader: R, offset: u32) -> Result<Self> {
		reader
			.seek(SeekFrom::Start(offset.into()))
			.map_err(|_| Error::Resource)?;

		let header = Header::read(&mut reader).map_err(|_| Error::Resource)?;

		Ok(Self {
			reader,
			block_base: offset + header.size,
			header,
		})
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
	size: u16,
	uncompressed_size: u16,
}
