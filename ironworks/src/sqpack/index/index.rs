use std::sync::{Arc, Mutex};

use binrw::BinRead;
use getset::CopyGetters;

use crate::sqpack::{
	Resource,
	error::{Error, Result},
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
	offset: u64,
	/// Estimated size of the target file, if known. This will typically err on
	/// the larger side, as files commonly have some amount of padding at the end.
	size: Option<u64>,
}

#[derive(Debug)]
pub struct Index<R> {
	repository: u8,
	category: u8,

	resource: Arc<R>,
	max_chunk: Mutex<Option<u16>>,
	chunks: Mutex<Vec<Arc<IndexChunk>>>,
}

impl<R: Resource> Index<R> {
	pub fn new(repository: u8, category: u8, resource: Arc<R>) -> Result<Self> {
		Ok(Self {
			repository,
			category,
			resource,
			max_chunk: None.into(),
			chunks: Vec::new().into(),
		})
	}

	pub fn find(&self, path: &str) -> Result<Location> {
		let location = self.chunks().find_map(|chunk| {
			let (index, chunk) = match chunk {
				Ok(value) => value,
				Err(error) => return Some(Err(error)),
			};

			match chunk.find(path) {
				Err(Error::FileNotFound) => None,
				Err(error) => Some(Err(error)),
				Ok((meta, size)) => Some(Ok(Location {
					chunk: index,
					data_file: meta.data_file_id,
					offset: meta.offset,
					size,
				})),
			}
		});

		location.unwrap_or(Err(Error::FileNotFound))
	}

	fn chunks(&self) -> impl Iterator<Item = Result<(u8, Arc<IndexChunk>)>> + '_ {
		// Get the max known chunk ID. If we don't know it, we want to loop the full potential ID space (u8).
		let guard = self.max_chunk.lock().unwrap();
		let max_chunk = guard.unwrap_or(256);
		drop(guard);

		(0u16..max_chunk).map_while(|index| {
			let index_usize = usize::from(index);
			let index_u8 = u8::try_from(index).unwrap();

			// If we've already loaded this chunk index, use that.
			let guard = self.chunks.lock().unwrap();
			if let Some(chunk) = guard.get(index_usize) {
				return Some(Ok((index_u8, chunk.clone())));
			}
			drop(guard);

			// Try to build a new chunk.
			let chunk = IndexChunk::new(
				self.repository,
				self.category,
				index.try_into().unwrap(),
				&*self.resource,
			);

			match chunk {
				// Found an index - save it out to the cache.
				Ok(Some(chunk)) => {
					let mut guard = self.chunks.lock().unwrap();
					guard.insert(index_usize, chunk.into());
					Some(Ok((index_u8, guard[index_usize].clone())))
				}

				// No index was found for this chunk - mark index as the max chunk point so we don't do that again.
				Ok(None) => {
					*self.max_chunk.lock().unwrap() = Some(index);
					None
				}

				// Surface errors
				Err(error) => Some(Err(error)),
			}
		})
	}
}

#[derive(Debug)]
enum IndexChunk {
	Index1(Index1),
	Index2(Index2),
}

impl IndexChunk {
	fn new<R: Resource>(
		repository: u8,
		category: u8,
		chunk: u8,
		resource: &R,
	) -> Result<Option<Self>> {
		if let Some(mut reader) = resource.index(repository, category, chunk)? {
			let index = Self::Index1(Index1::read(&mut reader)?);
			return Ok(Some(index));
		}

		if let Some(mut reader) = resource.index2(repository, category, chunk)? {
			let index = Self::Index2(Index2::read(&mut reader)?);
			return Ok(Some(index));
		}

		Ok(None)
	}

	fn find(&self, path: &str) -> Result<(FileMetadata, Option<u64>)> {
		match self {
			Self::Index1(index) => index.find(path),
			Self::Index2(index) => index.find(path),
		}
	}
}
