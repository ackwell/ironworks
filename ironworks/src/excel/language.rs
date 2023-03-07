use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{EnumIter, IntoEnumIterator};

/// Language of strings in Excel files.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Language {
	None = 0,
	Japanese = 1,
	English = 2,
	German = 3,
	French = 4,
	ChineseSimplified = 5,
	ChineseTraditional = 6,
	Korean = 7,
}

impl Language {
	/// Iterate over known language values.
	pub fn iter() -> <Self as IntoEnumIterator>::Iterator {
		<Self as IntoEnumIterator>::iter()
	}
}
