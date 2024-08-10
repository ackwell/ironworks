use super::error::Error;

// TODO: debug on this should probably be overwritten to (len,offset)
#[derive(Debug)]
pub struct SliceCursor<'a> {
	data: &'a [u8],
	offset: usize,
}

impl<'a> SliceCursor<'a> {
	pub fn new(data: &'a [u8]) -> Self {
		Self { data, offset: 0 }
	}

	pub fn eof(&self) -> bool {
		self.offset >= self.data.len()
	}

	pub fn peek(&self) -> Result<u8, Error> {
		match self.data.get(self.offset) {
			Some(&value) => Ok(value),
			None => Err(Error::UnexpectedEof),
		}
	}

	pub fn seek(&mut self, distance: usize) {
		self.offset += distance;
	}

	pub fn next(&mut self) -> Result<u8, Error> {
		let value = self.peek()?;
		self.seek(1);
		Ok(value)
	}

	pub fn take(&mut self, count: usize) -> Result<&'a [u8], Error> {
		let Some(value) = self.data.get(self.offset..(self.offset + count)) else {
			return Err(Error::UnexpectedEof);
		};
		self.seek(count);
		Ok(value)
	}

	pub fn take_until(&mut self, predicate: impl FnMut(&u8) -> bool) -> Result<&'a [u8], Error> {
		let length = match self.data.iter().skip(self.offset).position(predicate) {
			Some(position) => position,
			None => self.data.len() - self.offset,
		};

		self.take(length)
	}
}
