use std::{collections::HashSet, path::PathBuf};

use bevy::{
	asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
};
use ironworks::file::{mtrl, File};

use crate::material::BgMaterial;

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
	let samplers = material.samplers()?;

	// todo: handle the other texture types
	//       also this is atrocious, improve.
	let diffuse1_handle = samplers
		.iter()
		// TODO: Getting SamplerColorMap0 because The Chair:tm: uses it, this will need a lot of work to work in the general sense.
		.find(|sampler| sampler.id() == 0x1E6FEF9C)
		.map(|sampler| {
			let iw_path = format!("iw://{}", sampler.texture());
			let handle = load_context.get_handle::<_, Image>(&iw_path);
			dependencies.insert(iw_path);
			handle
		});

	let diffuse2_handle = samplers
		.iter()
		.find(|sampler| sampler.id() == 0x6968DF0A)
		.map(|sampler| {
			let iw_path = format!("iw://{}", sampler.texture());
			let handle = load_context.get_handle::<_, Image>(&iw_path);
			dependencies.insert(iw_path);
			handle
		});

	let material = BgMaterial {
		diffuse1: diffuse1_handle,
		diffuse2: diffuse2_handle,
	};

	let dependency_array = dependencies
		.into_iter()
		.map(|path| AssetPath::from(PathBuf::from(path)))
		.collect::<Vec<_>>();
	load_context.set_default_asset(LoadedAsset::new(material).with_dependencies(dependency_array));

	Ok(())
}
