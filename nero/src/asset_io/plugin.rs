use std::sync::{Arc, RwLock};

use bevy::{asset::create_platform_default_asset_io, prelude::*, tasks::IoTaskPool};
use ironworks::Ironworks;

use super::{asset_io::IronworksAssetIo, native::NativeIronworksPlugin};

// TODO: provide utility methods on this to somehow avoid people needing to manually set next state?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IronworksState {
	ResourceRequired,
	ResourceRequested,
	Ready,
}

// TODO: given this is used outside asset logic, should IW setup be seperated from iw's use in asset io?
pub type IronworksResource = Arc<RwLock<Ironworks>>;

pub struct IronworksAssetIoPlugin;

impl Plugin for IronworksAssetIoPlugin {
	fn build(&self, app: &mut App) {
		// Build the core instance of ironworks and store it as a resource for later.
		let ironworks = Arc::new(RwLock::new(Ironworks::new()));
		app.insert_resource(ironworks.clone());

		app.add_plugin(NativeIronworksPlugin);

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
