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
	file_structs::{BlockHeader, BlockInfo, FileHeader, Index, IndexHashTableValue},
	sqpack::{Category, Repository},
};

pub struct DatReader {
	repository: Repository,
	category: Category,

	// TODO: should i define these types separately?
	// TODO:
	index_table: HashMap<u64, IndexHashTableValue>,
}

impl DatReader {
	pub fn new(repository: Repository, category: Category) -> Self {
		return DatReader {
			index_table: build_index(&repository, &category),

			repository,
			category,
		};
	}

	pub fn read_file(&self, file_path: &str) -> Vec<u8> {
		// TODO: cache files? idk
		// TODO: index2
		// TODO: error handling
		let entry = self.get_index_entry(file_path).unwrap();

		let dat_path = build_sqpack_path(
			&self.repository,
			&self.category,
			0,
			"win32",
			&format!("dat{}", entry.data_file_id),
		);

		let mut file = File::open(dat_path).unwrap();
		file.seek(SeekFrom::Start(entry.offset.into())).unwrap();

		let header = FileHeader::read(&mut file).unwrap();

		let base_offset = entry.offset + header.file_info.size;
		let maybe_reader = header
			.blocks
			.iter()
			.map(|block_info| self.read_block(&mut file, base_offset, block_info))
			.reduce(|readers, reader| Box::new(readers.chain(reader)));

		// TODO: none should probs be an err?
		let mut reader = match maybe_reader {
			None => Box::new(io::empty()),
			Some(reader) => reader,
		};

		let mut buffer = Vec::new();
		let bytes_read = reader.read_to_end(&mut buffer).unwrap() as u32;

		if bytes_read != header.file_info.raw_file_size {
			panic!("todo: error handling");
		}

		return buffer;
	}

	fn read_block(
		&self,
		file: &mut File,
		base_offset: u32,
		block_info: &BlockInfo,
	) -> Box<dyn Read> {
		// Seek to the start of the block and read the raw bytes out.
		file.seek(SeekFrom::Start((base_offset + block_info.offset) as u64))
			.unwrap();

		let mut buffer = vec![0u8; block_info.size as usize];
		file.read_exact(&mut buffer).unwrap();

		// Build a base cursor and read the header.
		let mut cursor = Cursor::new(buffer);
		let header = BlockHeader::read(&mut cursor).unwrap();

		// If the block is uncompressed, we can return without further processing.
		// TODO: work out where to put this constant
		if header.uncompressed_size > 16000 {
			return Box::new(cursor);
		}

		// Set up deflate on the reader.
		return Box::new(DeflateDecoder::new(cursor));
	}

	fn get_index_entry(&self, file_path: &str) -> Option<&IndexHashTableValue> {
		// TODO: Error handling
		let (directory, filename) = file_path.rsplit_once('/').unwrap();

		let directory_hash = crc32(directory.as_bytes());
		let filename_hash = crc32(filename.as_bytes());

		let hash_key = (directory_hash as u64) << 32 | filename_hash as u64;

		return self.index_table.get(&hash_key);
	}
}

// TODO: handle index2
fn build_index(repository: &Repository, category: &Category) -> HashMap<u64, IndexHashTableValue> {
	// TODO: Deal with chunks
	let index_path = build_sqpack_path(repository, category, 0, "win32", "index");

	// Read the index file into memory before parsing to structs to avoid
	// thrashing seeks on-disk - we want the full data set anyway.
	// TODO: Error handling
	let buffer = fs::read(index_path).unwrap();
	let index = Index::read(&mut Cursor::new(buffer)).unwrap();

	// Build the lookup table
	// TODO: We probably need to include the chunk id in the map 'cus it's not in the bin
	let table: HashMap<_, _> = index
		.indexes
		.into_iter()
		.map(|entry| (entry.hash, entry.value))
		.collect();

	return table;
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
