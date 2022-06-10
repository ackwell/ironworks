use bevy::prelude::*;

use super::{
	mdl::MdlAssetLoader, mtrl::MtrlAssetLoader, proxy::ProxyAssetLoaderPlugin, tex::TexAssetLoader,
};

pub struct IronworksPlugin;

impl Plugin for IronworksPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(ProxyAssetLoaderPlugin)
			.init_asset_loader::<MdlAssetLoader>()
			.init_asset_loader::<MtrlAssetLoader>()
			.init_asset_loader::<TexAssetLoader>();
	}
}
