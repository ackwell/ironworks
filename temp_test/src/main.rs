use ironworks::{
	excel::Excel,
	ffxiv::{FsResource, Language, SqPackResource},
	sqpack::SqPack,
};
use ironworks_schema::saint_coinach::Provider;
use ironworks_sheets::{for_type, sheet::CompanionTransient};

fn main() -> anyhow::Result<()> {
	env_logger::init();

	iw_test()?;
	// stc_test()?;

	Ok(())
}

#[allow(dead_code)]
fn iw_test() -> anyhow::Result<()> {
	let sqpack_resource = FsResource::search().unwrap();
	let sqpack = SqPack::new(sqpack_resource);

	let resource = SqPackResource::new(&sqpack);
	// let excel = Excel::new(resource);
	let excel = Excel::with().language(Language::German).build(resource);

	let sheet = excel.sheet("CompanionTransient")?;
	let row = sheet.with().language(Language::English).row(101)?;
	let field = row.field(4)?;
	println!("{field:?}");

	// Gen sheet test
	let companion_transient = excel.sheet(for_type::<CompanionTransient>())?.row(101)?;
	println!("{companion_transient:#?}");

	let row = sheet.row(102)?;
	let field = row.field(4)?;
	println!("{field:?}");

	let sheet = excel.sheet("Behavior")?;
	let row = sheet.subrow(30016, 3)?;
	let field = row.field(4)?;
	println!("{field:?}");

	Ok(())
}

#[allow(dead_code)]
fn stc_test() -> anyhow::Result<()> {
	let schema = Provider::new().unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089fad484c25e491d3c37").unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089").unwrap();
	// let version = schema.version("refs/tags/69caa7e").unwrap();
	let version = schema.version("HEAD").unwrap();
	// let version = schema.version("master").unwrap();

	// let schema = version.sheet("RelicNote").unwrap();
	// let schema = version.sheet("ArrayEventHandler").unwrap();
	// let schema = version.sheet("PvPActionSort").unwrap();
	// let schema = version.sheet("Item").unwrap();
	let schema = version.sheet("CustomTalk").unwrap();

	println!("schema: {:#?}", schema);

	Ok(())
}
