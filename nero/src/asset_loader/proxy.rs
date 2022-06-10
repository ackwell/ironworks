use bevy::{
	asset::{AssetLoader, LoadedAsset},
	prelude::*,
	reflect::TypeUuid,
};
use ironworks::file::{eqdp, File};

pub struct ProxyAssetLoaderPlugin;
impl Plugin for ProxyAssetLoaderPlugin {
	fn build(&self, app: &mut App) {
		app.add_asset::<EquipmentDeformerParameter>()
			.init_asset_loader::<EqdpAssetLoader>();
	}
}

#[derive(Debug, Deref, TypeUuid)]
#[uuid = "7d4c008a-1b44-4bbb-8c63-01729ee73aed"]
pub struct EquipmentDeformerParameter(eqdp::EquipmentDeformerParameter);

#[derive(Default)]
pub struct EqdpAssetLoader;

impl AssetLoader for EqdpAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
		Box::pin(async move {
			let eqdp = <eqdp::EquipmentDeformerParameter as File>::read(bytes)?;
			load_context.set_default_asset(LoadedAsset::new(EquipmentDeformerParameter(eqdp)));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["eqdp"]
	}
}
