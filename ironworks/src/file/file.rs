use crate::error::Result;

/// A file that can be read from ironworks.
pub trait File: Sized {
	/// Build an instance of this file from the raw byte representation.
	fn read(data: Vec<u8>) -> Result<Self>;
}

impl File for Vec<u8> {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(data)
	}
}
