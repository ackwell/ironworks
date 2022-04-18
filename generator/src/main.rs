use std::{env::current_dir, fs};

use anyhow::Result;
use generate::generate_sheet;
use ironworks::{
	excel::Excel,
	ffxiv::{FsResource, SqpackResource},
	sqpack::SqPack,
};
use ironworks_schema::saint_coinach::Provider;

mod generate;

fn main() -> Result<()> {
	saint_coinach()?;

	Ok(())
}

// TODO: Seperate file and all that jazz.
fn saint_coinach() -> Result<()> {
	let provider = Provider::new()?;

	let sheet_name = "CustomTalk";

	// TODO: fetch updates to the provider to make sure it's fresh
	// TODO: arg for version?
	let version = provider.version("HEAD")?;
	let schema = version.sheet(sheet_name)?;

	// TODO: probably need args for this stuff too
	// TODO: this might be shareable across providers
	let sqpack = SqPack::new(FsResource::search().unwrap());
	let excel = Excel::new(SqpackResource::new(&sqpack));
	let sheet = excel.sheet(sheet_name)?;

	let sheet_code = generate_sheet(sheet_name, schema, sheet.columns()?);

	// TODO: this should probably be done at the next level up. also, more sanity lmao
	let folder = current_dir()?.join("gen_test").join("src");
	if !folder.exists() {
		fs::create_dir_all(&folder)?;
	}

	// TODO: this is a bit dupey with some of the generate logic - do we make generate return the file name to use in some way?
	let path = folder.join(format!("{sheet_name}.rs"));
	fs::write(path, sheet_code)?;

	Ok(())
}
