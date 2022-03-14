use ironworks_excel::{Excel, ExcelHeader, ExcelList, ExcelResource, ResourceResult};
use ironworks_ffxiv::SqPackFfxiv;
use ironworks_sqpack::SqPack;

fn main() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	let excel = Excel::new(SqPackResource::new(&sqpack));
	let sheet = excel.get_raw_sheet("CompanionTransient")?;
	let row = sheet.get_row(101);

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
	fn list(&self) -> ResourceResult<ExcelList> {
		let bytes = self.sqpack.read_file("exd/root.exl")?;
		let list = ExcelList::from_bytes(&bytes)?;
		Ok(list)
	}

	fn header(&self, sheet_name: &str) -> ResourceResult<ExcelHeader> {
		let bytes = self.sqpack.read_file(&format!("exd/{}.exh", sheet_name))?;
		let header = ExcelHeader::from_bytes(&bytes)?;
		Ok(header)
	}
}
