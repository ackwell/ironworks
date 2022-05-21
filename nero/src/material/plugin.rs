use bevy::prelude::*;

use super::bg::BgMaterial;

pub struct NeroMaterialPlugin;

impl Plugin for NeroMaterialPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(MaterialPlugin::<BgMaterial>::default());
	}
}
