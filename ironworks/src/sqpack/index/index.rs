use binrw::BinRead;
use getset::CopyGetters;

use crate::{
	error::{Error, ErrorValue, Result},
	sqpack::Resource,
};

use super::{index1::Index1, index2::Index2, shared::FileMetadata};

/// Specifier of a file location within a SqPack category.
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Location {
	/// SqPack chunk the file is in, i.e. `0000XX.win32.dat1`.
	chunk: u8,
	/// Data file the file is in, i.e. `000000.win32.datX`.
	data_file: u8,
	/// Offset within the targeted data file that the file starts at.
	offset: u32,
	/// Estimated size of the target file, if known. This will typically err on
	/// the larger side, as files commonly have some amount of padding at the end.
	size: Option<u32>,
}

#[derive(Debug)]
pub struct Index {
	chunks: Vec<IndexChunk>,
}

impl Index {
	pub fn new<R: Resource>(repository: u8, category: u8, resource: &R) -> Result<Self> {
		// TODO: This eager chunk lookup causes ZiPatch to eagerly parse every single prior patch to check if it exists. Is it possible to lazy this?
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
				Ok((meta, size)) => Some(Ok(Location {
					chunk: index.try_into().unwrap(),
					data_file: meta.data_file_id,
					offset: meta.offset,
					size,
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
	fn new<R: Resource>(repository: u8, category: u8, chunk: u8, resource: &R) -> Result<Self> {
		resource
			.index(repository, category, chunk)
			.and_then(|mut reader| {
				let file = Index1::read(&mut reader)?;
				Ok(IndexChunk::Index1(file))
			})
			.or_else(|_| {
				resource
					.index2(repository, category, chunk)
					.and_then(|mut reader| {
						let file = Index2::read(&mut reader)?;
						Ok(IndexChunk::Index2(file))
					})
			})
	}

	fn find(&self, path: &str) -> Result<(FileMetadata, Option<u32>)> {
		match self {
			Self::Index1(index) => index.find(path),
			Self::Index2(_index) => todo!("index2"),
		}
	}
}
