use std::{
	io,
	path::{Path, PathBuf},
	sync::{Arc, RwLock},
};

use bevy::{
	asset::{AssetIo, AssetIoError, AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	reflect::TypeUuid,
};
use ironworks::{
	ffxiv,
	file::{exl, File},
	sqpack::SqPack,
	Error, ErrorValue, Ironworks,
};

pub struct IronworksAssetIoPlugin;

impl Plugin for IronworksAssetIoPlugin {
	fn build(&self, app: &mut App) {
		let task_pool = app
			.world
			.get_resource::<bevy::tasks::IoTaskPool>()
			.expect("IoTaskPool resource not found")
			.0
			.clone();

		let default_io = bevy::asset::create_platform_default_asset_io(app);

		let ironworks = Arc::new(RwLock::new(Ironworks::new()));

		// TODO: Try this eagerly with search and otherwise defer to adding resource later with explicit path?
		ironworks
			.write()
			.unwrap()
			.add_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));

		let asset_io = IronworksAssetIo {
			default_io,
			ironworks,
		};

		app.insert_resource(AssetServer::new(asset_io, task_pool));
	}
}

pub struct IronworksPlugin;

impl Plugin for IronworksPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<ListAssetLoader>()
			.add_asset::<List>();
	}
}

struct IronworksAssetIo {
	default_io: Box<dyn AssetIo>,

	ironworks: Arc<RwLock<Ironworks>>,
}

impl AssetIo for IronworksAssetIo {
	fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
		if let Ok(ironworks_path) = path.strip_prefix("iw://") {
			Box::pin(async move {
				self.ironworks
					.read()
					.unwrap()
					.file::<Vec<u8>>(&ironworks_path.to_string_lossy())
					.map_err(|error| match error {
						Error::NotFound(ErrorValue::Path(path)) => {
							AssetIoError::NotFound(path.into())
						}
						other => AssetIoError::Io(io::Error::new(io::ErrorKind::Other, other)),
					})
			})
		} else {
			self.default_io.load_path(path)
		}
	}

	// The below just pass through to the base asset io, is it worth handling dirs, or changes for penumbra style resources?

	fn read_directory(
		&self,
		path: &std::path::Path,
	) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
		self.default_io.read_directory(path)
	}

	fn is_directory(&self, path: &Path) -> bool {
		self.default_io.is_directory(path)
	}

	fn watch_path_for_changes(&self, path: &Path) -> Result<(), AssetIoError> {
		self.default_io.watch_path_for_changes(path)
	}

	fn watch_for_changes(&self) -> Result<(), AssetIoError> {
		self.default_io.watch_for_changes()
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
			let list = exl::ExcelList::read(bytes.to_vec())?;

			load_context.set_default_asset(LoadedAsset::new(List(list)));

			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["exl"]
	}
}
