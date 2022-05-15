use std::borrow::Cow;

use crate::error::Result;

/// A file that can be read from ironworks.
pub trait File: Sized {
	/// Build an instance of this file from the raw byte representation.
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self>;
}

impl File for Vec<u8> {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		let cow: Cow<[u8]> = data.into();
		Ok(cow.into_owned())
	}
}
