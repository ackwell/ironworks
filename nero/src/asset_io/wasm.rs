use bevy::prelude::*;
use iyes_loopless::prelude::*;
use wasm_bindgen::prelude::*;

use super::IronworksState;

#[wasm_bindgen(module = "/src/asset_io/wasm.js")]
extern "C" {
	async fn pick_folder() -> JsValue;
}

pub struct WasmIronworksPlugin;

impl Plugin for WasmIronworksPlugin {
	fn build(&self, app: &mut App) {
		app.add_loopless_state(IronworksState::ResourceRequired)
			.add_enter_system(IronworksState::ResourceRequested, request_resource);
	}
}

fn request_resource(mut commands: Commands) {
	// TODO: This seems close to impossible to move out of this system.
	//       Tempted to say the approach should be reconsidered a little, and approached from an assetio reference. If wasm provides its own assetio impl, it would be able to natively await in load file calls - ergo it would be able to handle picking the folder and getting the array, and only tossing the slice into a fake "resource" just before it's needed.
	//       Doing the above would likely permit skipping a fair amount of the logic handled in the native impl - if the fake resource pops up the folder picker on first use, then a really basic file can be requested by the state change system (this one) to check if it works, and operate directly on handles.
	let future = async move {
		let something = pick_folder().await;
		info!("{:?}", something);
	};

	wasm_bindgen_futures::spawn_local(future);

	// TEMP
	commands.insert_resource(NextState(IronworksState::ResourceRequired))
}
