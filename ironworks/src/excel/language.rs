use std::fmt::Display;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

/// Language of strings in Excel files.
#[allow(missing_docs)]
#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	EnumIter,
	IntoPrimitive,
	TryFromPrimitive,
	Serialize,
	Deserialize,
)]
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
	TaiwanChinese = 8,
}

impl Language {
	/// Iterate over known language values.
	pub fn iter() -> <Self as IntoEnumIterator>::Iterator {
		<Self as IntoEnumIterator>::iter()
	}
}

impl Display for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Language::None => "None",
			Language::Japanese => "Japanese",
			Language::English => "English",
			Language::German => "German",
			Language::French => "French",
			Language::ChineseSimplified => "Chinese (Simplified)",
			Language::ChineseTraditional => "Chinese (Traditional)",
			Language::Korean => "Korean",
			Language::TaiwanChinese => "Taiwan Chinese (?)",
		})
	}
}
