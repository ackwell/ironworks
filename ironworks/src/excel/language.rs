use num_enum::{FromPrimitive, IntoPrimitive};
use strum::{EnumIter, IntoEnumIterator};

/// Language of strings in Excel files.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum Language {
	None = 0,
	Japanese = 1,
	English = 2,
	German = 3,
	French = 4,
	ChineseSimplified = 5,
	ChineseTraditional = 6,
	Korean = 7,
	ChineseTraditional2 = 8,

	/// An unknown language ID.
	///
	/// `Language::Unknown` should only be `match`ed with a wildcard (`_`)
	/// pattern. Values represented by `Unkown` may be promoted to new variants in
	/// future non-major versions.
	#[num_enum(catch_all)]
	Unknown(u8),
}

impl Language {
	/// Iterate over known language values.
	pub fn iter() -> <Self as IntoEnumIterator>::Iterator {
		<Self as IntoEnumIterator>::iter()
	}
}
