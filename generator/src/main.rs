use std::{
	fs,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;
use generate::generate_sheet;
use ironworks::{excel::Excel, ffxiv, sqpack::SqPack, Ironworks};
use ironworks_schema::{saint_coinach::Provider, Sheet as SchemaSheet};
use quote::{format_ident, quote};
use rust_embed::RustEmbed;
use toml_edit::Document;
use utility::unparse_tokens;

mod generate;
mod utility;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/template/src"]
struct Src;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/template/meta"]
struct Meta;

#[derive(Debug, Parser)]
struct Args {
	#[clap(short)]
	out_dir: Option<PathBuf>,

	#[clap(short)]
	game_path: Option<PathBuf>,
}

fn main() -> Result<()> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

	let args = Args::parse();

	// Clear out and prepare the target directory.
	let out_dir = args.out_dir.unwrap_or_else(|| "./ironworks_sheets".into());
	fs::remove_dir_all(&out_dir).ok();
	fs::create_dir_all(&out_dir)?;

	// We'll need a live Excel DB to generate sheets, set one up.
	let fs_resource = match args.game_path {
		Some(path) => ffxiv::FsResource::at(&path),
		None => ffxiv::FsResource::search().context("Game path search failed.")?,
	};
	let ironworks = Ironworks::new().resource(SqPack::new(fs_resource));
	let excel = Excel::new(&ironworks, ffxiv::Mapper::new());

	let (provider, version, schemas) = saint_coinach()?;
	let src_dir = build_scaffold(provider, version, excel.version()?, &out_dir)?;

	// Build the modules for sheets.
	let modules = schemas
		.into_iter()
		.filter_map(|schema| match excel.sheet(schema.name.clone()) {
			// Definitions might exist for sheets that no longer exist - ignore them.
			Err(ironworks::Error::NotFound(_)) => {
				log::warn!(
					"Sheet {} has definition but does not exist in game data.",
					schema.name
				);
				None
			}
			Err(error) => Some(Err(error)),
			Ok(sheet) => {
				let schema_name = schema.name.clone();
				let file = generate_sheet(schema, sheet.columns().ok()?);
				log::info!("Sheet {} generated.", schema_name);
				Some(Ok(file))
			}
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
		.map(|module| format_ident!("r#{}", module.name))
		.collect::<Vec<_>>();
	let module_tokens = quote! {
		#(mod #module_identifiers;)*
		#(pub use #module_identifiers::*;)*
	};
	fs::write(sheet_dir.join("mod.rs"), unparse_tokens(module_tokens))?;

	Ok(())
}

fn saint_coinach() -> Result<(String, String, Vec<SchemaSheet>)> {
	let provider = Provider::new()?;

	// TODO: fetch updates to the provider to make sure it's fresh
	// TODO: arg for version?
	let version = provider.version("HEAD")?;

	let schemas = version
		.sheet_names()?
		.iter()
		.map(|name| Ok(version.sheet(name)?))
		.collect::<Result<Vec<_>>>()?;

	Ok(("saint-coinach".into(), version.canonical(), schemas))
}

fn build_scaffold(
	provider: String,
	version: String,
	excel_version: String,
	out_dir: &Path,
) -> Result<PathBuf> {
	// Build and copy across the metadata
	let cargo_toml = Meta::get("Cargo.toml").unwrap();
	let mut document = std::str::from_utf8(&cargo_toml.data)?.parse::<Document>()?;
	if let Some(value) = document["package"]["version"].as_value_mut() {
		*value = format!("{}-{provider}", value.as_str().unwrap()).into()
	}
	fs::write(out_dir.join("Cargo.toml"), document.to_string())?;

	let readme = Meta::get("README.md").unwrap();
	let mut document = std::str::from_utf8(&readme.data)?.to_owned();
	document.push_str("||Version|\n|--|--|\n");
	document.push_str(&format!("|**{provider}**|{version}|\n"));
	document.push_str(&format!("|**Excel**|{excel_version}|\n"));
	fs::write(out_dir.join("README.md"), document)?;

	let src_dir = out_dir.join("src");
	fs::create_dir(&src_dir)?;

	// Copy out all the supporting source files
	for path in Src::iter() {
		fs::write(src_dir.join(path.as_ref()), Src::get(&path).unwrap().data)?;
	}

	Ok(src_dir)
}
