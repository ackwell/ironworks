use ironworks_excel::{Excel, ExcelResource, ResourceResult};
use ironworks_ffxiv::SqPackFfxiv;
use ironworks_sqpack::SqPack;

fn main() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	let excel = Excel::new(SqPackResource::new(&sqpack));
	let sheet = excel.get_raw_sheet("CompanionTransient")?;
	let row = sheet.get_row(101)?;

	println!("{:#?}", row);

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

	fn page(&self, sheet_name: &str, start_id: u32) -> ResourceResult<Vec<u8>> {
		// TODO: HANDLE LANG! "en" is invalid on unlang sheets, plus we want multiple langs and shit
		let bytes = self
			.sqpack
			.read_file(&format!("exd/{}_{}_en.exd", sheet_name, start_id))?;
		Ok(bytes)
	}
}
