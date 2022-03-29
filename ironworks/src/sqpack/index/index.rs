use std::rc::Rc;

use binrw::BinRead;

use crate::{
	error::{Error, Result},
	sqpack::Resource,
};

use super::{index1::Index1, index2::Index2};

// do i just trait this?
// or would making it a trait make the reader, and then the sqpack, need to generic over it
// in that case wrapper makes more sense i guess

// tempted to say index owns chunks and then it can return file locations like the old one but with less wiring

// with the binary reading stuff this should probably be split up into a few files

#[derive(Debug)]
pub struct Index<R> {
	resource: Rc<R>,
}

impl<R: Resource> Index<R> {
	pub fn new(repository: u8, category: u8, resource: Rc<R>) -> Result<Self> {
		let chunks = (0u8..=255)
			.map_while(|chunk_id| {
				match IndexChunk::new(repository, category, chunk_id, resource.as_ref()) {
					Err(Error::NotFound) => None,
					Err(error) => Some(Err(error)),
					Ok(chunk) => Some(Ok(chunk)),
				}
			})
			.collect::<Result<Vec<_>>>()?;

		println!("uuuuuh... something? {chunks:#?}");

		Ok(Self { resource })
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
}
