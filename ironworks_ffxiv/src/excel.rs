use ironworks_excel::{Excel, ExcelOptions, ExcelResource, ResourceResult};
use ironworks_sqpack::{Error, SqPack};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Extension trait that adds methods to construct Excel instances pre-configured
/// to read resources from SqPack files in the FFXIV conventional layout.
pub trait ExcelSqPack<'a> {
	/// Configure an Excel instance reading resources from a SqPack database.
	fn sqpack(sqpack: &'a SqPack) -> Self;

	/// Configure an Excel instance reading resources from a SqPack database,
	/// with additional options.
	fn sqpack_with_options(sqpack: &'a SqPack, options: ExcelOptions) -> Self;
}

impl<'a> ExcelSqPack<'a> for Excel<'a> {
	fn sqpack(sqpack: &'a SqPack) -> Self {
		Excel::new(SqPackResource::new(sqpack))
	}

	fn sqpack_with_options(sqpack: &'a SqPack, options: ExcelOptions) -> Self {
		Excel::with_options(SqPackResource::new(sqpack), options)
	}
}

#[derive(Debug)]
struct SqPackResource<'a> {
	sqpack: &'a SqPack,
}

impl<'a> SqPackResource<'a> {
	fn new(sqpack: &'a SqPack) -> Self {
		Self { sqpack }
	}
}

impl ExcelResource for SqPackResource<'_> {
	fn list(&self) -> ResourceResult<Vec<u8>> {
		let bytes = self.sqpack.read_file("exd/root.exl")?;
		Ok(bytes)
	}

	fn header(&self, sheet_name: &str) -> ResourceResult<Vec<u8>> {
		let bytes = self.sqpack.read_file(&format!("exd/{}.exh", sheet_name))?;
		Ok(bytes)
	}

	fn page(&self, sheet_name: &str, start_id: u32, language_id: u8) -> ResourceResult<Vec<u8>> {
		let language_suffix = match Language::try_from(language_id) {
			Ok(Language::None) => "",
			Ok(Language::Japanese) => "_ja",
			Ok(Language::English) => "_en",
			Ok(Language::German) => "_de",
			Ok(Language::French) => "_fr",
			Ok(Language::ChineseSimplified) => "_chs",
			Ok(Language::ChineseTraditional) => "_cht",
			Ok(Language::Korean) => "_kr",
			Err(_) => {
				// TODO: better error type. should this lib have an error?
				return Err(
					Error::InvalidDatabase(format!("Invalid language ID {}", language_id)).into(),
				);
			}
		};

		let bytes = self.sqpack.read_file(&format!(
			"exd/{}_{}{}.exd",
			sheet_name, start_id, language_suffix
		))?;
		Ok(bytes)
	}
}

/// Language IDs used by FFXIV Excel files.
#[allow(missing_docs)]
#[derive(TryFromPrimitive, IntoPrimitive, Debug)]
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
