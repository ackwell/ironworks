use std::{cmp, io};

pub trait TakeSeekableExt: io::Read + io::Seek + Sized {
	fn take_seekable(self, limit: u64) -> io::Result<TakeSeekable<Self>>;
}

impl<R: io::Read + io::Seek> TakeSeekableExt for R {
	fn take_seekable(mut self, limit: u64) -> io::Result<TakeSeekable<Self>> {
		let offset = self.stream_position()?;
		Ok(TakeSeekable {
			inner: self,
			current: 0,
			offset,
			limit,
		})
	}
}

/// Reader adapter which limits the bytes read from an underlying reader, and provides seeking capabilities.
///
/// This struct is created by calling `TakeSeekableExt::take_seekable` on a seekable reader.
#[derive(Debug)]
pub struct TakeSeekable<R> {
	inner: R,
	current: u64,
	offset: u64,
	limit: u64,
}

impl<R: io::Read> io::Read for TakeSeekable<R> {
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		// Don't call into inner reader at all at EOF because it may still block
		if self.current >= self.limit {
			return Ok(0);
		}

		let remaining = self.limit - self.current;

		let max = cmp::min(buffer.len() as u64, remaining) as usize;
		let bytes_read = self.inner.read(&mut buffer[..max])?;
		assert!(
			bytes_read as u64 <= remaining,
			"number of read bytes exceeds limit"
		);
		self.current += bytes_read as u64;
		Ok(bytes_read)
	}
}

impl<S: io::Seek> io::Seek for TakeSeekable<S> {
	fn seek(&mut self, position: io::SeekFrom) -> io::Result<u64> {
		let (base, position) = match position {
			io::SeekFrom::Start(position) => {
				let inner_offset = self
					.inner
					.seek(io::SeekFrom::Start(self.offset + position))?;
				self.current = inner_offset - self.offset;
				return Ok(self.current);
			}
			io::SeekFrom::Current(position) => (self.current, position),
			io::SeekFrom::End(position) => (self.limit, position),
		};

		let ioffset = i128::from(base)
			.checked_add(position.into())
			.ok_or_else(|| {
				io::Error::new(
					io::ErrorKind::InvalidInput,
					"invalid seek to an overflowing position",
				)
			})?;

		if ioffset < 0 {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"invalid seek to a negative position",
			));
		}

		let inner_offset = self.inner.seek(io::SeekFrom::Start(
			self.offset + u64::try_from(ioffset).unwrap(),
		))?;

		self.current = inner_offset - self.offset;
		Ok(self.current)
	}
}
