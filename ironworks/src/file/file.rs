use crate::{FileStream, error::Result};

/// A file that can be read from ironworks.
pub trait File: Sized {
	/// Build an instance of this file from the raw byte representation.
	fn read(stream: impl FileStream) -> Result<Self>;
}

impl File for Vec<u8> {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		let mut buffer = Vec::new();
		stream.read_to_end(&mut buffer)?;
		Ok(buffer)
	}
}
