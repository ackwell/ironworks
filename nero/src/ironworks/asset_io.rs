use std::{
	io,
	path::{Path, PathBuf},
	sync::{Arc, RwLock},
};

use bevy::{
	asset::{create_platform_default_asset_io, AssetIo, AssetIoError, BoxedFuture},
	prelude::*,
	tasks::{AsyncComputeTaskPool, IoTaskPool, Task},
};
use futures_lite::future;
use ironworks::{ffxiv, sqpack::SqPack, ErrorValue, Ironworks};
use iyes_loopless::prelude::*;
use rfd::{AsyncFileDialog, FileHandle};

pub struct IronworksAssetIoPlugin;

impl Plugin for IronworksAssetIoPlugin {
	fn build(&self, app: &mut App) {
		// Build the core instance of ironworks and store it as a resource for later.
		let ironworks = Arc::new(RwLock::new(Ironworks::new()));
		app.insert_resource(ironworks.clone());

		// Try to find a game install, skipping straight to ready if one was found.
		let state = match ffxiv::FsResource::search() {
			Some(resource) => {
				ironworks
					.write()
					.unwrap()
					.add_resource(SqPack::new(resource));
				IronworksState::Ready
			}
			None => IronworksState::ResourceRequired,
		};
		app.add_loopless_state(state);

		// Set up infrastructure for adding resources.
		app.add_enter_system(IronworksState::ResourceRequested, request_resource)
			.add_system(poll_path_selection);

		// Build up the AssetIo implementation and insert it.
		let asset_io = IronworksAssetIo {
			default_io: create_platform_default_asset_io(app),
			ironworks,
		};

		let task_pool = app
			.world
			.get_resource::<IoTaskPool>()
			.expect("IoTaskPool resource not found")
			.0
			.clone();

		app.insert_resource(AssetServer::new(asset_io, task_pool));
	}
}

// TODO: provide utility methods on this to somehow avoid people needing to manually set next state?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IronworksState {
	ResourceRequired,
	ResourceRequested,
	Ready,
}

#[derive(Component)]
struct PathSelection(Task<Option<FileHandle>>);

fn request_resource(mut commands: Commands, task_pool: Res<AsyncComputeTaskPool>) {
	let future = AsyncFileDialog::new().pick_folder();
	let task = task_pool.spawn(future);
	commands.spawn().insert(PathSelection(task));
}

fn poll_path_selection(
	mut commands: Commands,
	mut tasks: Query<(Entity, &mut PathSelection)>,
	ironworks: Res<Arc<RwLock<Ironworks>>>,
) {
	for (entity, mut task) in tasks.iter_mut() {
		// Poll the task once to check if there's a response from the dialog.
		if let Some(response) = future::block_on(future::poll_once(&mut task.0)) {
			let state = match response {
				// A path was selected, add it as a resource and mark ready.
				// TODO: try to sanity check the path somehow? Wrong path will just blow up. Might be able to query if one of the .ver files exists?
				Some(file_handle) => {
					let resource = ffxiv::FsResource::at(file_handle.path());
					ironworks
						.write()
						.unwrap()
						.add_resource(SqPack::new(resource));

					IronworksState::Ready
				}

				// Interaction cancelled, jump back to the base state.
				None => IronworksState::ResourceRequired,
			};

			commands.insert_resource(NextState(state));

			// The task was completed, remove the marker entity.
			commands.entity(entity).despawn();
		}
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
