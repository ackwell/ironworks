use binrw::BinRead;
use flate2::read::DeflateDecoder;
use std::{
	collections::HashMap,
	fs::{self, File},
	io::{self, Cursor, Read, Seek, SeekFrom},
	path::PathBuf,
};
use thiserror::Error;

use crate::{
	crc::crc32,
	file_structs::{BlockHeader, BlockInfo, FileHeader, Index, IndexHashTableValue},
};

// TODO: this should probably be in own file
#[derive(Error, Debug)]
pub enum SqPackError {
	#[error("invalid sqpack path \"{0}\"")]
	InvalidPath(String),

	#[error("unknown repository \"{repository}\" in sqpack path \"{path}\"")]
	UnknownRepository { path: String, repository: String },

	#[error("unknown category \"{category}\" in sqpack path \"{path}\"")]
	UnknownCategory { path: String, category: String },
}

#[derive(Debug)]
pub struct SqPack {
	pub repositories: HashMap<String, PathBuf>,
	pub categories: HashMap<String, u8>,

	pub default_repository: String,
}

impl SqPack {
	pub fn temp_test(&self, sqpack_path: &str) -> Result<(), SqPackError> {
		let path = self.parse_path(sqpack_path)?;

		let repository_path = self.repositories.get(&path.repository).ok_or_else(|| {
			SqPackError::UnknownRepository {
				path: path.path.clone(),
				repository: path.repository.clone(),
			}
		})?;

		let category_id =
			self.categories
				.get(&path.category)
				.ok_or_else(|| SqPackError::UnknownCategory {
					path: path.path.clone(),
					category: path.category.clone(),
				})?;

		println!("repo: {:?}, cat: {}", repository_path, category_id);

		// TODO: cache readers
		let reader = SqPackReader::new(
			Repository {
				id: 0,
				name: path.repository,
				path: repository_path.to_owned(),
			},
			Category {
				id: *category_id,
				name: path.category,
			},
		);

		let exlt = String::from_utf8(reader.read_file(sqpack_path)).unwrap();

		println!("EXLT: {}", exlt);

		return Ok(());
	}

	fn parse_path(&self, sqpack_path: &str) -> Result<SqPackPath, SqPackError> {
		// TODO: Look into itertools or something?
		let lower = sqpack_path.to_lowercase();
		let split = lower.splitn(3, '/').take(2).collect::<Vec<&str>>();
		let (category, mut repository) = match split[..] {
			[category, repository] => (category, repository),
			_ => return Err(SqPackError::InvalidPath(sqpack_path.to_string())),
		};

		if !self.repositories.contains_key(repository) {
			repository = &self.default_repository
		}

		return Ok(SqPackPath {
			category: String::from(category),
			repository: String::from(repository),
			path: lower,
		});
	}
}

// TODO: probs should call this path and namespace on consume
// TODO: I mean realistically this can just be an internal tuple?
#[derive(Debug)]
pub struct SqPackPath {
	path: String,
	category: String,
	repository: String,
}

// note: not calling this category as it a) kinda handles both repo and category, and b) would conflict with a potential category metadata struct
struct SqPackReader {
	repository: Repository,
	category: Category,

	// TODO: should i define these types separately?
	// TODO:
	index_table: HashMap<u64, IndexHashTableValue>,
}

impl SqPackReader {
	fn new(repository: Repository, category: Category) -> Self {
		return SqPackReader {
			index_table: build_index(&repository, &category),

			repository,
			category,
		};
	}

	fn read_file(&self, file_path: &str) -> Vec<u8> {
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

struct Repository {
	name: String,
	id: u8,
	path: PathBuf,
}

struct Category {
	name: String,
	id: u8,
}
