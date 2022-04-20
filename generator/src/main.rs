use std::{
	env::current_dir,
	fs,
	path::{Path, PathBuf},
};

use anyhow::Result;
use generate::generate_sheet;
use ironworks::{
	excel::Excel,
	ffxiv::{FsResource, SqpackResource},
	sqpack::SqPack,
};
use ironworks_schema::{saint_coinach::Provider, Sheet as SchemaSheet};
use quote::{format_ident, quote};
use rust_embed::RustEmbed;
use utility::unparse_tokens;

mod generate;
mod utility;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/template/src"]
struct Src;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/template/meta"]
struct Meta;

fn main() -> Result<()> {
	// TODO: output dir should be configurable
	// TODO: more sanity lmao
	// Clear out and prepare the target directory.
	let out_dir = current_dir()?.join("gen_test");
	fs::remove_dir_all(&out_dir).ok();
	fs::create_dir_all(&out_dir)?;

	let src_dir = build_scaffold(&out_dir)?;

	// TODO: configurable lookup dir
	// We'll need a live Excel DB to generate sheets, set one up.
	let sqpack = SqPack::new(FsResource::search().unwrap());
	let excel = Excel::new(SqpackResource::new(&sqpack));

	// Build the modules for sheets.
	let modules = saint_coinach()?
		.into_iter()
		.map(|schema| -> Result<_, ironworks::Error> {
			let sheet = excel.sheet(&schema.name)?;
			let file = generate_sheet(schema, sheet.columns()?);
			Ok(file)
		})
		.collect::<Result<Vec<_>, _>>()?;

	// Write out the modules into the scaffold.
	let sheet_dir = src_dir.join("sheet");
	fs::create_dir(&sheet_dir)?;

	for module in &modules {
		fs::write(
			sheet_dir.join(format!("{}.rs", module.name)),
			&module.content,
		)?;
	}

	// Build the mod.rs file
	let module_identifiers = modules
		.iter()
		.map(|module| format_ident!("{}", module.name))
		.collect::<Vec<_>>();
	let module_tokens = quote! {
		#(mod #module_identifiers;)*
		#(pub use #module_identifiers::*;)*
	};
	fs::write(sheet_dir.join("mod.rs"), unparse_tokens(module_tokens))?;

	Ok(())
}

// TODO: Seperate file and all that jazz.
fn saint_coinach() -> Result<Vec<SchemaSheet>> {
	let provider = Provider::new()?;

	// TODO: fetch updates to the provider to make sure it's fresh
	// TODO: arg for version?
	let version = provider.version("HEAD")?;

	// TODO: TEMP
	let sheets = ["BaseParam", "CharaMakeType", "CustomTalk"];

	sheets
		.iter()
		.map(|name| Ok(version.sheet(name)?))
		.collect::<Result<Vec<_>>>()
}

fn build_scaffold(out_dir: &Path) -> Result<PathBuf> {
	// Build and copy across the metadata
	let cargo_toml = Meta::get("Cargo.toml").unwrap();
	// TODO: edit the name/version/etc
	fs::write(out_dir.join("Cargo.toml"), cargo_toml.data)?;

	let src_dir = out_dir.join("src");
	fs::create_dir(&src_dir)?;

	// Copy out all the supporting source files
	for path in Src::iter() {
		fs::write(src_dir.join(path.as_ref()), Src::get(&path).unwrap().data)?;
	}

	Ok(src_dir)
}
