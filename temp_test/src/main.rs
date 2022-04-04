use std::io::Read;

use ironworks::sqpack::FfxivFsResource;
use ironworks_excel::{Excel, ExcelOptions, RowOptions};
use ironworks_ffxiv::{ExcelSqPack, Language, SqPackFfxiv};
use ironworks_schema_saint_coinach::SaintCoinachSchema;
use ironworks_sqpack::SqPack;

fn main() -> anyhow::Result<()> {
	env_logger::init();

	iw_test()?;
	// stc_test()?;
	// excel_test()?;

	Ok(())
}

#[allow(dead_code)]
fn iw_test() -> anyhow::Result<()> {
	let sqpack_resource = FfxivFsResource::search().unwrap();
	let sqpack = ironworks::sqpack::SqPack::new(sqpack_resource);

	// let mut exl = sqpack.read("exd/root.exl")?;
	// let ded = sqpack.read("exd/fsdfsd/xd/roodsft.exl")?;

	// let mut buffer = vec![];
	// exl.read_to_end(&mut buffer)?;
	// let string = String::from_utf8_lossy(&buffer);
	// println!("exl: {string}");

	let excel = ironworks::excel::Excel::new(TestExcelResource { sqpack: &sqpack });
	let sheet = excel.sheet("CompanionTransient")?;
	let row = sheet.row(101)?;

	println!("row: {row:?}");

	Ok(())
}

struct TestExcelResource<'a, R> {
	sqpack: &'a ironworks::sqpack::SqPack<R>,
}
impl<R: ironworks::sqpack::Resource> ironworks::excel::Resource for TestExcelResource<'_, R> {
	type List = ironworks::sqpack::File<R::Dat>;
	fn list(&self) -> Result<Self::List, ironworks::Error> {
		self.sqpack.read("exd/root.exl")
	}

	type Header = ironworks::sqpack::File<R::Dat>;
	fn header(&self, sheet: &str) -> Result<Self::Header, ironworks::Error> {
		self.sqpack.read(&format!("exd/{sheet}.exh"))
	}

	type Page = ironworks::sqpack::File<R::Dat>;
	fn page(
		&self,
		sheet: &str,
		start_id: u32,
		language_id: u8,
	) -> Result<Self::Page, ironworks::Error> {
		// TODO: language
		self.sqpack.read(&format!("exd/{sheet}_{start_id}_en.exd"))
	}
}

#[allow(dead_code)]
fn stc_test() -> anyhow::Result<()> {
	let schema = SaintCoinachSchema::new().unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089fad484c25e491d3c37").unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089").unwrap();
	// let version = schema.version("refs/tags/69caa7e").unwrap();
	let version = schema.version("HEAD").unwrap();
	// let version = schema.version("master").unwrap();

	// let schema = version.schema("RelicNote").unwrap();
	// let schema = version.schema("ArrayEventHandler").unwrap();
	// let schema = version.schema("PvPActionSort").unwrap();
	let schema = version.schema("Item").unwrap();

	println!("schema: {:#?}", schema);

	Ok(())
}

#[allow(dead_code)]
fn excel_test() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	let excel = Excel::sqpack_with_options(
		&sqpack,
		ExcelOptions {
			default_language: Language::German.into(),
		},
	);

	let sheet = excel.sheet_reader("CompanionTransient")?;
	let row = sheet.row_with_options(101, RowOptions::new().language(Language::English))?;
	let field = row.field(4)?;
	println!("{:?}", field);

	let row = sheet.row(102)?;
	let field = row.field(4)?;
	println!("{:?}", field);

	let sheet = excel.sheet_reader("Behavior")?;
	let row = sheet.subrow(30016, 3)?;
	let field = row.field(4)?;
	println!("{:?}", field);

	Ok(())
}
