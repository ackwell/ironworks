use std::{
	fs,
	io::{self, Seek},
};

use crate::sqpack;

#[derive(Debug)]
pub struct PatchSection {
	pub offset: usize,
	pub size: usize,
	pub reader: sqpack::BlockStream<io::BufReader<fs::File>>,
}

#[derive(Debug)]
pub struct SectionStream {
	sections: Vec<PatchSection>,

	index: usize,
	position: usize,
	dirty: bool,
}

impl SectionStream {
	pub fn new(mut sections: Vec<PatchSection>) -> Self {
		// Ensure that sections are sorted in offset order.
		// TODO: Consider whether this invariant should be upheld by the caller instead?
		sections.sort_by_key(|section| section.offset);

		Self {
			sections,

			index: 0,
			position: 0,
			dirty: false,
		}
	}

	fn dirty_seek(&mut self, position: usize) {
		if position == self.position {
			return;
		}
		self.position = position;
		self.dirty = true;
	}
}

impl io::Read for SectionStream {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let max_index = self.sections.len() - 1;

		let mut section = &mut self.sections[self.index];

		// If we're at EOF, bail with no data.
		if self.position == section.offset + section.size && self.index == max_index {
			return Ok(0);
		}

		let mut dirty = self.dirty;
		self.dirty = false;

		// If the current position has moved outside the current section, find the
		// next target and mark dirty to ensure we align with a seek.
		if self.position < section.offset || self.position >= section.offset + section.size {
			let point = self
				.sections
				.partition_point(|section| section.offset <= self.position);
			self.index = point - 1;
			section = &mut self.sections[self.index];

			dirty = true;
		}

		// If a seek has been performed but not applied, ensure it gets applied
		// before we perform a read.
		if dirty {
			// NOTE: All section BlockStreams are configured with the same origin, but
			// may have various subsets of the necessary blocks - so seeks act within
			// a consistent position space equivalent to our own.
			let offset = u64::try_from(self.position).unwrap();
			section.reader.seek(io::SeekFrom::Start(offset))?;
		}

		let bytes_read = section.reader.read(buf)?;
		self.position += bytes_read;
		Ok(bytes_read)
	}
}

// TODO: This is pretty much 1:1 with BlockStream, consider making a wrapper?
impl io::Seek for SectionStream {
	fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
		let (base, offset) = match pos {
			io::SeekFrom::Start(position) => {
				self.dirty_seek(position.try_into().unwrap());
				return Ok(position);
			}

			io::SeekFrom::Current(position) => (self.position, position),
			io::SeekFrom::End(position) => {
				let base = match self.sections.last() {
					Some(section) => section.offset + section.size,
					None => 0,
				};
				(base, position)
			}
		};

		let Some(position) = base.checked_add_signed(offset.try_into().unwrap()) else {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"invalid seek to a negative or overflowing position",
			));
		};

		self.dirty_seek(position);

		Ok(position.try_into().unwrap())
	}
}
