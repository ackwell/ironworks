use binrw::BinRead;

use crate::{
	error::{Error, Result},
	sqpack::Resource,
};

use super::{index1::Index1, index2::Index2, shared::FileMetadata};

#[derive(Debug)]
pub struct Location {
	chunk: u8,
	data_file: u8,
	offset: u32,
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
					Err(Error::NotFound) => None,
					Err(error) => Some(Err(error)),
					Ok(chunk) => Some(Ok(chunk)),
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
				Err(Error::NotFound) => None,
				Err(error) => Some(Err(error)),
				Ok(meta) => Some(Ok(Location {
					chunk: index.try_into().unwrap(),
					data_file: meta.data_file_id,
					offset: meta.offset,
				})),
			});

		match matching_chunk {
			None => Err(Error::NotFound),
			Some(result) => result,
		}
	}
}

#[derive(Debug)]
enum IndexChunk {
	Index1(Index1),
	Index2(Index2),
}

impl IndexChunk {
	fn new<R: Resource>(repository: u8, category: u8, chunk: u8, resource: &R) -> Result<Self> {
		resource
			.index(repository, category, chunk)
			.and_then(|mut reader| {
				let file = Index1::read(&mut reader).map_err(|_| Error::Resource)?;
				Ok(IndexChunk::Index1(file))
			})
			.or_else(|_| {
				resource
					.index2(repository, category, chunk)
					.and_then(|mut reader| {
						let file = Index2::read(&mut reader).map_err(|_| Error::Resource)?;
						Ok(IndexChunk::Index2(file))
					})
			})
	}

	fn find(&self, path: &str) -> Result<FileMetadata> {
		match self {
			Self::Index1(index) => index.find(path),
			Self::Index2(index) => todo!("index2"),
		}
	}
}
