use std::{collections::HashSet, path::PathBuf};

use bevy::{
	asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	utils::HashMap,
};
use ironworks::file::{mtrl, File};

use crate::render::{Material, MaterialKind};

#[derive(Default)]
pub struct MtrlAssetLoader;

impl AssetLoader for MtrlAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move { load_mtrl(bytes, load_context) })
	}

	fn extensions(&self) -> &[&str] {
		&["mtrl"]
	}
}

fn load_mtrl<'a>(
	bytes: &'a [u8],
	load_context: &'a mut LoadContext<'_>,
) -> Result<(), anyhow::Error> {
	// TODO: this pattern will probably crop up a bunch. abstract it and the handle logic as a helper?
	let mut dependencies = HashSet::<String>::new();

	let material = <mtrl::Material>::read(bytes)?;

	let samplers = material
		.samplers()
		.iter()
		.map(|sampler| {
			let iw_path = format!("iw://{}", sampler.texture());
			let handle = load_context.get_handle::<_, Image>(&iw_path);
			// TODO: Not a fan of mutating in a map
			dependencies.insert(iw_path);
			(sampler.id(), handle)
		})
		.collect::<HashMap<_, _>>();

	// TODO: get the above hooked up again
	let material = Material {
		kind: MaterialKind::Bg,
		samplers,
	};

	let dependency_array = dependencies
		.into_iter()
		.map(|path| AssetPath::from(PathBuf::from(path)))
		.collect::<Vec<_>>();
	load_context.set_default_asset(LoadedAsset::new(material).with_dependencies(dependency_array));

	Ok(())
}
