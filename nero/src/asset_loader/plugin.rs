use bevy::prelude::*;

use super::{
	mdl::MdlAssetLoader, mtrl::MtrlAssetLoader, sklb::SklbAssetLoader, tex::TexAssetLoader,
};

pub struct IronworksPlugin;

impl Plugin for IronworksPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<MdlAssetLoader>()
			.init_asset_loader::<MtrlAssetLoader>()
			.init_asset_loader::<SklbAssetLoader>()
			.init_asset_loader::<TexAssetLoader>();
	}
}
