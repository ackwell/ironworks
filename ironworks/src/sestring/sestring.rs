use binrw::{binread, NullString};

/// SeString representation and utilities.
#[binread]
#[derive(Debug)]
pub struct SeString {
	inner: NullString,
}

impl ToString for SeString {
	fn to_string(&self) -> String {
		self.inner.clone().to_string()
	}
}

// TODO: all the rest of the sestring handling. maybe under a ffxiv lock for some payloads? and a sestring one for general non-payload logic?
