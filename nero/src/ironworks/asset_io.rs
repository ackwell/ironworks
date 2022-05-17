use std::{
	io,
	path::{Path, PathBuf},
	sync::{Arc, RwLock},
};

use bevy::{
	asset::{AssetIo, AssetIoError, BoxedFuture},
	prelude::*,
};
use ironworks::{ffxiv, sqpack::SqPack, ErrorValue, Ironworks};
use iyes_loopless::prelude::*;

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

		let state = match /* ffxiv::FsResource::search() */None::<ffxiv::FsResource> {
			Some(res) => {
				ironworks.write().unwrap().add_resource(SqPack::new(res));
				IronworksState::Ready
			}
			None => IronworksState::NeedsResource,
		};
		app.add_loopless_state(state);

		let asset_io = IronworksAssetIo {
			default_io,
			ironworks,
		};

		app.insert_resource(AssetServer::new(asset_io, task_pool));
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IronworksState {
	NeedsResource,
	Ready,
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
						ironworks::Error::NotFound(ErrorValue::Path(path)) => {
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
