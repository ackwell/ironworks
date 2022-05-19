use bevy::prelude::*;
use iyes_loopless::prelude::*;
use wasm_bindgen::prelude::*;

use super::IronworksState;

#[wasm_bindgen(module = "/src/asset_io/wasm.js")]
extern "C" {
	fn pick_folder();
}

pub struct WasmIronworksPlugin;

impl Plugin for WasmIronworksPlugin {
	fn build(&self, app: &mut App) {
		app.add_loopless_state(IronworksState::ResourceRequired);

		pick_folder();
	}
}
