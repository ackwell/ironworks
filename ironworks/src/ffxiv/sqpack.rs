use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
	error::{Error, ErrorValue, Result},
	sqpack,
};

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

// TODO: Probably should be FfxivSqPackResource
/// Resource adapter pre-configured to read Excel files from a SqPack instance,
/// laid out in the expected FFXIV format.
#[derive(Debug)]
pub struct SqpackResource<'s, R> {
	sqpack: &'s sqpack::SqPack<R>,
}

impl<'s, R: sqpack::Resource> SqpackResource<'s, R> {
	/// Configure a resource instance with a given SqPack handler.
	pub fn new(sqpack: &'s sqpack::SqPack<R>) -> Self {
		Self { sqpack }
	}
}

#[cfg(feature = "excel")]
use crate::excel;

#[cfg(feature = "excel")]
impl<R: sqpack::Resource> excel::Resource for SqpackResource<'_, R> {
	type List = sqpack::File<R::Dat>;
	fn list(&self) -> Result<Self::List> {
		self.sqpack.file("exd/root.exl")
	}

	type Header = sqpack::File<R::Dat>;
	fn header(&self, sheet: &str) -> Result<Self::Header> {
		self.sqpack.file(&format!("exd/{sheet}.exh"))
	}

	type Page = sqpack::File<R::Dat>;
	fn page(&self, sheet: &str, start_id: u32, language_id: u8) -> Result<Self::Page> {
		use Language as L;

		let language = Language::try_from(language_id)
			.map_err(|_| Error::NotFound(ErrorValue::Other(format!("language {language_id}"))))?;
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

		self.sqpack
			.file(&format!("exd/{sheet}_{start_id}{language_suffix}.exd"))
	}
}
