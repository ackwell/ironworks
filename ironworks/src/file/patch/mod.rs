// TEMP
#![allow(missing_docs, dead_code)]

mod chunk;
mod zipatch;

pub use {
	chunk::Chunk,
	zipatch::{ChunkIterator, ZiPatch},
};
