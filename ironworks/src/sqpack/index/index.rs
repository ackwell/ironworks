use binrw::BinRead;

use crate::{
	error::{Error, ErrorValue, Result},
	sqpack::Resource,
};

use super::{index1::Index1, index2::Index2, shared::FileMetadata};

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
	pub fn new<R: Resource>(path_metadata: &R::PathMetadata, resource: &R) -> Result<Self> {
		let chunks = (0u8..=255)
			.map_while(
				|chunk_id| match IndexChunk::new(path_metadata, chunk_id, resource) {
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
	Index1(Index1),
	Index2(Index2),
}

impl IndexChunk {
	fn new<R: Resource>(path_metadata: &R::PathMetadata, chunk: u8, resource: &R) -> Result<Self> {
		resource
			.index(path_metadata, chunk)
			.and_then(|mut reader| {
				let file = Index1::read(&mut reader)?;
				Ok(IndexChunk::Index1(file))
			})
			.or_else(|_| {
				resource
					.index2(path_metadata, chunk)
					.and_then(|mut reader| {
						let file = Index2::read(&mut reader)?;
						Ok(IndexChunk::Index2(file))
					})
			})
	}

	fn find(&self, path: &str) -> Result<FileMetadata> {
		match self {
			Self::Index1(index) => index.find(path),
			Self::Index2(_index) => todo!("index2"),
		}
	}
}
