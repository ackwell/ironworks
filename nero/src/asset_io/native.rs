use bevy::{
	prelude::*,
	tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use ironworks::{ffxiv, sqpack::SqPack};
use iyes_loopless::prelude::*;
use rfd::{AsyncFileDialog, FileHandle};

use super::plugin::{IronworksResource, IronworksState};

pub struct NativeIronworksPlugin;

impl Plugin for NativeIronworksPlugin {
	fn build(&self, app: &mut App) {
		let ironworks = app.world.get_resource::<IronworksResource>().unwrap();

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
			.add_system(poll_path_selection.run_in_state(IronworksState::ResourceRequested));
	}
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
	ironworks: Res<IronworksResource>,
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
