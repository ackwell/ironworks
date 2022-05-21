use bevy::{prelude::*, reflect::TypeUuid};

use super::bg::BgMaterial;

pub const BG_SHADER_HANDLE: HandleUntyped =
	HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 4234278907890421);

pub struct NeroMaterialPlugin;

impl Plugin for NeroMaterialPlugin {
	fn build(&self, app: &mut App) {
		let mut assets = app.world.resource_mut::<Assets<_>>();
		assets.set_untracked(BG_SHADER_HANDLE, Shader::from_wgsl(include_str!("bg.wgsl")));

		app.add_plugin(MaterialPlugin::<BgMaterial>::default());
	}
}
