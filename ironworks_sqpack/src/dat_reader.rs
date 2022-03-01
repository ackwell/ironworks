use std::{
	collections::HashMap,
	fs::{self, File},
	io::{self, Cursor, Read, Seek, SeekFrom},
	path::PathBuf,
};

use binrw::BinRead;
use flate2::read::DeflateDecoder;

use crate::{
	crc::crc32,
	errors::{Result, SqPackError},
	file_structs::{BlockHeader, BlockInfo, FileHeader, Index, IndexHashTableValue},
	sqpack::{Category, Repository},
};

pub struct DatReader<'a> {
	repository: &'a Repository,
	category: &'a Category,

	// TODO: should i define these types separately?
	index_table: HashMap<u64, IndexHashTableValue>,
}

impl<'a> DatReader<'a> {
	pub fn new(repository: &'a Repository, category: &'a Category) -> Result<Self> {
		return Ok(DatReader {
			index_table: build_index(repository, category)?,

			repository,
			category,
		});
	}

	pub fn read_file(&self, file_path: &str) -> Result<Vec<u8>> {
		// TODO: cache files? idk
		// TODO: index2
		let entry = self.get_index_entry(file_path)?;

		let dat_path = build_sqpack_path(
			self.repository,
			self.category,
			0,
			"win32",
			&format!("dat{}", entry.data_file_id),
		);

		let mut file = File::open(&dat_path)?;
		file.seek(SeekFrom::Start(entry.offset.into()))?;

		let header = FileHeader::read(&mut file).map_err(|_| {
			SqPackError::InvalidData(format!(
				"File header in \"{}\" at {:#x}",
				dat_path.to_string_lossy(),
				entry.offset
			))
		})?;

		let base_offset = entry.offset + header.file_info.size;

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

		if bytes_read != header.file_info.raw_file_size {
			panic!("todo: error handling");
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
			.map_err(|_| SqPackError::InvalidData(format!("Block header at {:#x}", offset)))?;

		// If the block is uncompressed, we can return without further processing.
		// TODO: work out where to put this constant
		if header.uncompressed_size > 16000 {
			return Ok(Box::new(cursor));
		}

		// Set up deflate on the reader.
		return Ok(Box::new(DeflateDecoder::new(cursor)));
	}

	fn get_index_entry(&self, file_path: &str) -> Result<&IndexHashTableValue> {
		let (directory, filename) = file_path
			.rsplit_once('/')
			.ok_or_else(|| SqPackError::InvalidPath(file_path.to_owned()))?;

		let directory_hash = crc32(directory.as_bytes());
		let filename_hash = crc32(filename.as_bytes());

		let hash_key = (directory_hash as u64) << 32 | filename_hash as u64;

		return self
			.index_table
			.get(&hash_key)
			.ok_or_else(|| SqPackError::NotFound(file_path.to_owned()));
	}
}

// TODO: handle index2
fn build_index(
	repository: &Repository,
	category: &Category,
) -> Result<HashMap<u64, IndexHashTableValue>> {
	// TODO: Deal with chunks
	let index_path = build_sqpack_path(repository, category, 0, "win32", "index");

	// Read the index file into memory before parsing to structs to avoid
	// thrashing seeks on-disk - we want the full data set anyway.
	let buffer = fs::read(&index_path)?;
	let index = Index::read(&mut Cursor::new(buffer)).map_err(|_| {
		SqPackError::InvalidData(format!(
			"Index data in \"{}\"",
			index_path.to_string_lossy(),
		))
	})?;

	// Build the lookup table
	// TODO: We probably need to include the chunk id in the map 'cus it's not in the bin
	let table: HashMap<_, _> = index
		.indexes
		.into_iter()
		.map(|entry| (entry.hash, entry.value))
		.collect();

	return Ok(table);
}

fn build_sqpack_path(
	repository: &Repository,
	category: &Category,
	chunk_id: u8,
	platform: &str,
	file_type: &str,
) -> PathBuf {
	let mut path = PathBuf::new();
	path.push(&repository.path);
	path.push(format!(
		"{:02x}{:02x}{:02x}.{}.{}",
		category.id, repository.id, chunk_id, platform, file_type
	));
	return path;
}
