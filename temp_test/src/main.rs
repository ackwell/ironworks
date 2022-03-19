use ironworks_excel::{Excel, ExcelOptions, ExcelResource, ResourceResult, RowOptions};
use ironworks_ffxiv::SqPackFfxiv;
use ironworks_sqpack::SqPack;
use num_enum::{IntoPrimitive, TryFromPrimitive};

fn main() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	// let excel = Excel::new(SqPackResource::new(&sqpack));
	let excel = Excel::with_options(
		SqPackResource::new(&sqpack),
		ExcelOptions {
			default_language: Language::English.into(),
		},
	);
	let sheet = excel.get_raw_sheet("CompanionTransient")?;
	let row = sheet.get_row_with_options(101, &RowOptions::new().language(Language::German))?;
	let field = row.read_field(4)?;

	println!("{:?}", field);

	Ok(())
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
			Err(_) => return Err(anyhow::format_err!("Invalid language ID {}", language_id)),
		};

		let bytes = self.sqpack.read_file(&format!(
			"exd/{}_{}{}.exd",
			sheet_name, start_id, language_suffix
		))?;
		Ok(bytes)
	}
}

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Language {
	None = 0,
	Japanese = 1,
	English = 2,
	German = 3,
	French = 4,
	ChineseSimplified = 5,
	ChineseTraditional = 6,
	Korean = 7,
}
