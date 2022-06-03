use bevy::{
	prelude::*,
	render::{
		render_asset::RenderAssetPlugin, render_component::ExtractComponentPlugin,
		render_phase::AddRenderCommand, render_resource::SpecializedMeshPipelines, RenderApp,
		RenderStage,
	},
};

use super::{
	material::Material,
	pipeline::{queue, Draw, Pipeline, RenderMode},
};

// TODO: should this be opaque in the long run?

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
	fn build(&self, app: &mut App) {
		app.add_asset::<Material>()
			.add_plugin(ExtractComponentPlugin::<Handle<Material>>::default())
			.add_plugin(RenderAssetPlugin::<Material>::default());

		app.sub_app_mut(RenderApp)
			.add_render_command::<RenderMode, Draw>()
			.init_resource::<Pipeline>()
			.init_resource::<SpecializedMeshPipelines<Pipeline>>()
			.add_system_to_stage(RenderStage::Queue, queue);
	}
}
