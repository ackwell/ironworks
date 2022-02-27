use binrw::BinRead;
use flate2::read::DeflateDecoder;
use glob::glob;
use std::{
	collections::HashMap,
	fs::File,
	io::{Cursor, Read, Seek, SeekFrom},
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
				repository: path.repository,
			}
		})?;

		let category_id =
			self.categories
				.get(&path.category)
				.ok_or_else(|| SqPackError::UnknownCategory {
					path: path.path.clone(),
					category: path.category,
				})?;

		println!("repo: {:?}, cat: {}", repository_path, category_id);

		// TODO: Should probably do both index and index2 seperately, and maybe at the same time?
		// TODO: i mean... TODO: index2 lmao
		// TODO: chunks, ex (does ex matter, really? in a repo?), platform?
		// TODO: also this fn is basically a "get_file", this should be done lazily and cached
		let mut index_path = PathBuf::new();
		index_path.push(repository_path);
		index_path.push(format!("{:02x}????.*.index", category_id));

		let indexes = glob(&index_path.to_string_lossy())
			.unwrap()
			.map(|path| path.unwrap())
			.collect::<Vec<PathBuf>>();

		// TODO this is dirty, do stuff better
		if indexes.len() != 1 {
			panic!(
				"too many results in index lookup, fix this shit (chunks?) {:?}",
				indexes
			)
		}

		let index_path = &indexes[0];

		// TODO: error handling lmao
		let bytes = std::fs::read(index_path).unwrap();
		let mut reader = Cursor::new(bytes);
		let index = Index::read(&mut reader).unwrap();

		// TODO: should probably use the into_iter given we'll be discarding the rest... right?
		let index_table = index.indexes.iter().map(|entry| (entry.hash, &entry.value));
		// let index_table = index
		// 	.indexes
		// 	.into_iter()
		// 	.map(|entry| (entry.hash, entry.value));
		let index_hash_table = HashMap::<_, _>::from_iter(index_table);

		println!(
			"binrw: {} entries, [0]: {:#?}",
			index.indexes.len(),
			index.indexes[0]
		);

		let (directory, filename) = path.path.rsplit_once('/').unwrap();

		let directory_hash = crc32(directory.as_bytes());
		let filename_hash = crc32(filename.as_bytes());

		let hash_key = (directory_hash as u64) << 32 | filename_hash as u64;

		println!("{}, {} : {}", directory_hash, filename_hash, hash_key);

		let hash_entry = index_hash_table.get(&hash_key).unwrap();

		println!("value: {:#?}", hash_entry);

		// -----

		// TODO: do this properly
		let mut index_path = PathBuf::new();
		index_path.push(repository_path);
		index_path.push(format!("{:02x}0000.win32.dat0", category_id));

		let mut dat_file = File::open(index_path).unwrap();
		let mut file = self.read_file(&mut dat_file, hash_entry);

		let mut exlt = String::new();
		file.read_to_string(&mut exlt).unwrap();

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

	// TODO: why are these even on &self tbh
	// TODO: should this function own reading the file on real disk? - tempted to say yes?
	fn read_file(&self, file: &mut File, index_entry: &IndexHashTableValue) -> Box<dyn Read> {
		// Seek to the start of the file and read the basic file header.
		file.seek(SeekFrom::Start(index_entry.offset.into()))
			.unwrap();

		// This is reading directly out of `File` - keep an eye on performance and look into buffering if this starts to bottleneck
		let header = FileHeader::read(file).unwrap();
		println!("header: {:#?}", header);

		let base_offset = index_entry.offset + header.file_info.size;
		let reader = header
			.blocks
			.iter()
			.map(|block_info| self.read_block(file, base_offset, block_info))
			.reduce(|readers, reader| Box::new(readers.chain(reader)));

		return match reader {
			None => Box::new(std::io::empty()),
			Some(reader) => reader,
		};
	}

	// TODO: this sig is a bit meh, combine the file stuff?
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
}

// TODO: probs should call this path and namespace on consume
// TODO: I mean realistically this can just be an internal tuple?
#[derive(Debug)]
pub struct SqPackPath {
	path: String,
	category: String,
	repository: String,
}
