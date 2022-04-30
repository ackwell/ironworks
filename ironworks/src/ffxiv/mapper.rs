use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Language of strings in Excel files.
#[allow(missing_docs)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
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

/// Path mapper pre-configured for FFXIV file locations.
#[derive(Debug)]
#[non_exhaustive]
pub struct Mapper;

impl Mapper {
	/// Create a new mapper.
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for Mapper {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(feature = "excel")]
use crate::excel;

#[cfg(feature = "excel")]
impl excel::Mapper for Mapper {
	// fn version(&self) -> Result<String> {
	// 	self.sqpack.version("exd/root.exl")
	// }

	fn exl(&self) -> String {
		"exd/root.exl".into()
	}

	fn exh(&self, sheet: &str) -> String {
		format!("exd/{sheet}.exh")
	}

	fn exd(&self, sheet: &str, start_id: u32, language_id: u8) -> String {
		use Language as L;

		let language = Language::try_from(language_id).unwrap_or(Language::None);
		let language_suffix = match language {
			L::None => "",
			L::Japanese => "_ja",
			L::English => "_en",
			L::German => "_de",
			L::French => "_fr",
			L::ChineseSimplified => "_chs",
			L::ChineseTraditional => "_cht",
			L::Korean => "_kr",
		};

		format!("exd/{sheet}_{start_id}{language_suffix}.exd")
	}
}
