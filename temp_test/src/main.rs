use ironworks_excel::{Excel, ExcelOptions, RowOptions};
use ironworks_ffxiv::{ExcelSqPack, Language, SqPackFfxiv};
use ironworks_schema_saint_coinach::test;
use ironworks_sqpack::SqPack;

fn main() -> anyhow::Result<()> {
	env_logger::init();

	stc_test()?;
	// excel_test()?;

	Ok(())
}

#[allow(dead_code)]
fn stc_test() -> anyhow::Result<()> {
	test();
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
