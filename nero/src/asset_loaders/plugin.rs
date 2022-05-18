use bevy::{
	asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	reflect::TypeUuid,
};
use ironworks::file::{exl, File};

use super::{mdl::MdlAssetLoader, tex::TexAssetLoader};

pub struct IronworksPlugin;

impl Plugin for IronworksPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<ListAssetLoader>()
			.init_asset_loader::<MdlAssetLoader>()
			.init_asset_loader::<TexAssetLoader>()
			.add_asset::<List>();
	}
}

// ???
// TODO: i'll need to newtype most iw stuff for asset handling, should i deref them?
#[derive(Debug, TypeUuid)]
#[uuid = "3584bf2d-97c2-42a1-a2a2-858f8bc4840b"]
pub struct List(pub exl::ExcelList);

// ??? temp stuff to test it works
#[derive(Default)]
struct ListAssetLoader;

impl AssetLoader for ListAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move {
			// TODO: this is pretty wasteful - none of the readers except vecu8 need an owned copy. that said, i'm not sure how best to handle the vec case - do i blindly copy and lose the passthrough benefit for other consumers? am i able to abuse asref or into<cow to allow [u8]|vecu80>vecu8?
			let list = exl::ExcelList::read(bytes)?;

			load_context.set_default_asset(LoadedAsset::new(List(list)));

			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["exl"]
	}
}
