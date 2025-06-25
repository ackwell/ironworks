use std::{error::Error, io};

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
	#[error("malformed data")]
	Malformed(#[source] Box<dyn Error + Send + Sync + 'static>),

	#[error("I/O")]
	Io(#[source] io::Error),
}

pub trait FromReader: Sized {
	fn read(reader: impl io::Read + io::Seek) -> Result<Self, ReadError>;
}

impl FromReader for Vec<u8> {
	fn read(mut reader: impl io::Read + io::Seek) -> Result<Self, ReadError> {
		let mut buffer = Vec::new();
		reader.read_to_end(&mut buffer).map_err(ReadError::Io)?;
		Ok(buffer)
	}
}
