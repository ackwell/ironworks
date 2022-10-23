use std::{
	io::SeekFrom,
	sync::{Arc, Mutex},
};

use binrw::{binread, BinRead};
use derivative::Derivative;

use crate::{error::Result, file::File, FileStream};

use super::chunk::Chunk;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ZiPatch {
	#[derivative(Debug = "ignore")]
	stream: Arc<Mutex<Box<dyn FileStream>>>,
}

impl ZiPatch {
	pub fn todo_name_me_iterate_chunks(&self) -> ChunkIterator {
		// TODO: ctor? - need to store that base offset somewhere so it doesn't just nightmare fuel me
		ChunkIterator {
			stream: self.stream.clone(),
			offset: 12,
			complete: false,
		}
	}
}

impl File for ZiPatch {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		// Check the magic in the header
		let mut magic = [0u8; 12];
		stream.read_exact(&mut magic)?;

		if &magic != b"\x91ZIPATCH\x0D\x0A\x1A\x0A" {
			todo!("error message")
		}

		// Rest of the file is chunks that we'll read lazily.
		Ok(Self {
			// TODO: I'm really not happy with this incantation
			stream: Arc::new(Mutex::new(Box::new(stream))),
		})
	}
}

pub struct ChunkIterator {
	stream: Arc<Mutex<Box<dyn FileStream>>>,
	offset: u64,
	complete: bool,
}

impl Iterator for ChunkIterator {
	type Item = Chunk;

	fn next(&mut self) -> Option<Self::Item> {
		if self.complete {
			return None;
		}

		let mut stream = self.stream.lock().expect("TODO poison message");

		// Seek to last known offset - in a tight loop this is effectively a noop, but need to make sure if there's stuff jumping around.
		// TODO: lots of jumping around would be catastrophic for read performance - it'd be nice to be able to request something cloneable, so i.e. file handles could be cloned between chunk iterators, rather than trying to share access to a single one - but i'm not sure how to mode that without major refactors.
		stream
			.seek(SeekFrom::Start(self.offset))
			.expect("TODO this shouldn't happen, right? what do");

		let container = ChunkContainer::read(&mut *stream).expect(
			"TODO i'm really not sure what to do about all these results in an option return type",
		);

		// Update the offset to the new position
		self.offset = stream.stream_position().expect("TODO FUCK ME");

		// TODO: check the hash? is it worth it?

		// If we just read an EOF chunk, mark this iterator as complete.
		if let Chunk::EndOfFile = container.chunk {
			self.complete = true;
		}

		Some(container.chunk)
	}
}

#[binread]
#[br(big)]
#[derive(Debug)]
struct ChunkContainer {
	#[br(temp)]
	size: u32,

	#[br(pad_size_to = size + 4)]
	chunk: Chunk,

	hash: u32,
}
