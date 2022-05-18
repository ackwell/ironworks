use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::IronworksState;

pub struct WasmIronworksPlugin;

impl Plugin for WasmIronworksPlugin {
	fn build(&self, app: &mut App) {
		app.add_loopless_state(IronworksState::ResourceRequired);
	}
}
