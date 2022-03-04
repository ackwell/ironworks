use std::{
	fs::File,
	io::{self, Cursor, Read, Seek, SeekFrom},
};

use binrw::BinRead;
use flate2::read::DeflateDecoder;

use crate::{
	error::{Error, Result},
	file_struct::{BlockHeader, BlockInfo, FileHeader},
	index::Index,
	sqpack::{Category, Repository},
	utility::build_file_path,
};

#[derive(Debug)]
pub struct DatReader<'a> {
	repository: &'a Repository,
	category: &'a Category,

	chunks: Vec<Index>,
}

impl<'a> DatReader<'a> {
	pub fn new(repository: &'a Repository, category: &'a Category) -> Result<Self> {
		let mut chunks: Vec<Index> = vec![];

		for chunk_id in 0u8..=255 {
			match Index::new(repository, category, chunk_id)? {
				None => continue,
				Some(index) => chunks.push(index),
			};
		}

		return Ok(DatReader {
			chunks,

			repository,
			category,
		});
	}

	pub fn read_file(&self, sqpack_path: &str) -> Result<Vec<u8>> {
		// TODO: cache files? idk
		let location = self
			.chunks
			.iter()
			.find_map(|index| {
				index.get_file_location(sqpack_path).map_or_else(
					|err| match err {
						Error::NotFound(_) => None,
						_ => Some(Err(err)),
					},
					|location| Some(Ok(location)),
				)
			})
			.unwrap_or_else(|| Err(Error::NotFound(sqpack_path.to_owned())))?;

		let dat_path = build_file_path(
			self.repository,
			self.category,
			location.chunk_id,
			"win32",
			&format!("dat{}", location.data_file_id),
		);

		let mut file = File::open(&dat_path)?;
		file.seek(SeekFrom::Start(location.offset.into()))?;

		let header = FileHeader::read(&mut file).map_err(|_| {
			Error::InvalidData(format!(
				"File header in \"{}\" at {:#x}",
				dat_path.to_string_lossy(),
				location.offset
			))
		})?;

		let base_offset = location.offset + header.size;

		let mut reader = header
			.blocks
			.iter()
			.map(|block_info| self.read_block(&mut file, base_offset, block_info))
			.try_fold(
				Box::new(io::empty()) as Box<dyn Read>,
				|readers, result| match result {
					Ok(reader) => Ok(Box::new(readers.chain(reader)) as Box<dyn Read>),
					Err(error) => Err(error),
				},
			)?;

		let mut buffer = Vec::new();
		let bytes_read = reader.read_to_end(&mut buffer)? as u32;

		if bytes_read != header.raw_file_size {
			return Err(Error::InvalidData(format!(
				"Expected \"{}\" to have size {}, got {}",
				sqpack_path.to_owned(),
				header.raw_file_size,
				bytes_read
			)));
		}

		return Ok(buffer);
	}

	fn read_block(
		&self,
		file: &mut File,
		base_offset: u32,
		block_info: &BlockInfo,
	) -> Result<Box<dyn Read>> {
		// Seek to the start of the block and read the raw bytes out.
		let offset = base_offset + block_info.offset;
		file.seek(SeekFrom::Start(offset as u64))?;

		let mut buffer = vec![0u8; block_info.size as usize];
		file.read_exact(&mut buffer)?;

		// Build a base cursor and read the header.
		let mut cursor = Cursor::new(buffer);
		let header = BlockHeader::read(&mut cursor)
			.map_err(|_| Error::InvalidData(format!("Block header at {:#x}", offset)))?;

		// If the block is uncompressed, we can return without further processing.
		// TODO: work out where to put this constant
		if header.uncompressed_size > 16000 {
			return Ok(Box::new(cursor));
		}

		// Set up deflate on the reader.
		return Ok(Box::new(DeflateDecoder::new(cursor)));
	}
}
