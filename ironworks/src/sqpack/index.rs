use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	file::index,
	sqpack::Resource,
};

#[derive(Debug)]
pub struct Location {
	pub chunk: u8,
	pub data_file: u8,
	pub offset: u32,
}

#[derive(Debug)]
pub struct Index {
	chunks: Vec<IndexChunk>,
}

impl Index {
	pub fn new<R: Resource>(repository: u8, category: u8, resource: &R) -> Result<Self> {
		let chunks = (0u8..=255)
			.map_while(
				|chunk_id| match IndexChunk::new(repository, category, chunk_id, resource) {
					Err(Error::NotFound(_)) => None,
					other => Some(other),
				},
			)
			.collect::<Result<Vec<_>>>()?;

		Ok(Self { chunks })
	}

	pub fn find(&self, path: &str) -> Result<Location> {
		let matching_chunk = self
			.chunks
			.iter()
			.enumerate()
			.find_map(|(index, chunk)| match chunk.find(path) {
				Err(Error::NotFound(_)) => None,
				Err(error) => Some(Err(error)),
				Ok(meta) => Some(Ok(Location {
					chunk: index.try_into().unwrap(),
					data_file: meta.data_file_id,
					offset: meta.offset,
				})),
			});

		match matching_chunk {
			None => Err(Error::NotFound(ErrorValue::Path(path.into()))),
			Some(result) => result,
		}
	}
}

#[derive(Debug)]
enum IndexChunk {
	Index1(index::Index),
	Index2,
}

impl IndexChunk {
	fn new<R: Resource>(repository: u8, category: u8, chunk: u8, resource: &R) -> Result<Self> {
		resource
			.index(repository, category, chunk)
			.and_then(|mut reader| {
				let file = index::Index::read(&mut reader)?;
				Ok(IndexChunk::Index1(file))
			})
			.or_else(|_| {
				resource
					.index2(repository, category, chunk)
					.map(|mut _reader| IndexChunk::Index2)
			})
	}

	fn find(&self, path: &str) -> Result<index::FileMetadata> {
		match self {
			Self::Index1(index) => index.find(path),
			Self::Index2 => todo!("index2"),
		}
	}
}
