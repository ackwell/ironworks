#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum MacroKind {
	NewLine,

	Unknown(u8),
}

impl From<u8> for MacroKind {
	fn from(value: u8) -> Self {
		match value {
			0x10 => Self::NewLine,

			other => Self::Unknown(other),
		}
	}
}
