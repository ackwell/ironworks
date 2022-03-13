use ironworks_excel::{ExcelList, ExcelResource, ResourceResult};
use ironworks_ffxiv::SqPackFfxiv;
use ironworks_sqpack::SqPack;

fn main() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	let resource = SqpackResource { sqpack };
	let list = resource.list().unwrap();

	println!("has item: {}", list.has_sheet("Item"));

	Ok(())
}

struct SqpackResource {
	sqpack: SqPack,
}

impl ExcelResource for SqpackResource {
	fn list(&self) -> ResourceResult<ExcelList> {
		let bytes = self.sqpack.read_file("exd/root.exl")?;
		let list = ExcelList::from_bytes(&bytes)?;
		Ok(list)
	}
}
