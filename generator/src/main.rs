use std::{env::current_dir, fs};

use anyhow::Result;
use generate::generate_sheet;
use ironworks::{
	excel::Excel,
	ffxiv::{FsResource, SqpackResource},
	sqpack::SqPack,
};
use ironworks_schema::saint_coinach::Provider;
use quote::{format_ident, quote};
use rust_embed::RustEmbed;
use utility::unparse_tokens;

mod generate;
mod utility;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/template/src"]
struct Src;

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
	fs::remove_dir_all(&folder)?;
	fs::create_dir_all(&folder)?;

	// Copy out all the supporting source files
	for path in Src::iter() {
		fs::write(folder.join(path.as_ref()), Src::get(&path).unwrap().data)?;
	}

	// TODO: this is a bit dupey with some of the generate logic - do we make generate return the file name to use in some way?
	let generated_folder = folder.join("generated");
	fs::create_dir(&generated_folder)?;
	fs::write(
		generated_folder.join(format!("{sheet_name}.rs")),
		sheet_code,
	)?;

	// todo: fucking lmao
	let module_identifier = format_ident!("{sheet_name}");
	let module_contents = quote! {
		mod #module_identifier;

		pub use #module_identifier::*;
	};
	fs::write(
		generated_folder.join("mod.rs"),
		unparse_tokens(module_contents),
	)?;

	Ok(())
}
