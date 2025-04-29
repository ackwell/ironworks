use std::{fs, io};

use binrw::BinRead;

use crate::{file::patch, sqpack};

#[derive(Debug)]
enum IteratorState {
	Pending,
	Active,
	Complete,
}

/// Iterator over the chunks within a patch file.
///
/// Chunks are read lazily from the source stream over the course of iteration.
#[derive(Debug)]
pub struct ChunkIterator {
	stream: io::BufReader<fs::File>,
	state: IteratorState,
}

impl ChunkIterator {
	pub fn new(stream: io::BufReader<fs::File>) -> Self {
		ChunkIterator {
			stream,
			state: IteratorState::Pending,
		}
	}
}

impl Iterator for ChunkIterator {
	type Item = sqpack::Result<patch::Chunk>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.state {
			// We've already hit EOF, fuse.
			IteratorState::Complete => return None,

			// Read past the header on first iteration.
			IteratorState::Pending => {
				if let Err(error) = patch::Header::read(&mut self.stream) {
					return Some(Err(error.into()));
				}
				self.state = IteratorState::Active;
			}

			IteratorState::Active => {}
		}

		let chunk = match patch::ChunkContainer::read(&mut self.stream) {
			Err(error) => return Some(Err(error.into())),
			Ok(container) => container.chunk,
		};

		if matches!(chunk, patch::Chunk::EndOfFile) {
			self.state = IteratorState::Complete;
		}

		Some(Ok(chunk))
	}
}
