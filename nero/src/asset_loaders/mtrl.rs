use bevy::{
	asset::{AssetLoader, BoxedFuture, LoadContext},
	prelude::*,
};

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
	Ok(())
}
