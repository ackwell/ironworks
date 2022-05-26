#![allow(clippy::module_inception)]

mod asset_io;
mod asset_loader;
mod camera;
mod material;
mod tool;

use asset_io::{IronworksAssetIoPlugin, IronworksState};
use asset_loader::IronworksPlugin;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use camera::CameraPlugin;
use iyes_loopless::prelude::*;
use material::NeroMaterialPlugin;
use strum::IntoEnumIterator;
use tool::{Tool, ToolPlugins};

fn main() {
	App::new()
		// Ironworks
		.add_plugins_with(DefaultPlugins, |group| {
			group.add_before::<bevy::asset::AssetPlugin, _>(IronworksAssetIoPlugin)
		})
		.add_plugin(IronworksPlugin)
		// UI
		.insert_resource(WinitSettings::desktop_app())
		.add_plugin(EguiPlugin)
		.add_system(ui_need_ironworks_resource.run_not_in_state(IronworksState::Ready))
		// TODO: the label here should probably be a const somewhere sensible
		// Running before "UI" to ensure it's always outermost.
		.add_system(ui_toolbox.run_in_state(IronworksState::Ready).before("ui"))
		.add_plugin(ToolPlugins)
		// 3D
		.add_plugin(CameraPlugin)
		.add_plugin(NeroMaterialPlugin)
		// Asset test stuff
		.add_enter_system(IronworksState::Ready, asset_test)
		// Done
		.run();
}

fn asset_test(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// mut meshes: ResMut<Assets<Mesh>>,
	// mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// 2D texture test
	// commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	// commands.spawn_bundle(SpriteBundle {
	// 	texture: asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/texture/s1t0_a0_flag1_d.tex"),
	// 	..default()
	// });

	// 3D model test
	// commands
	// 	.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/bgparts/s1t0_z0_flg3.mdl"));
	// commands
	// 	.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_char1.mdl"));
	// commands
	// 	.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_flr2.mdl"));
	commands
		.spawn_scene(asset_server.load("iw://bg/ffxiv/wil_w1/dun/w1d5/bgparts/w1d5_q1_bre4b.mdl"));
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
}

fn ui_need_ironworks_resource(
	mut commands: Commands,
	mut egui_context: ResMut<EguiContext>,
	ironworks_state: Res<CurrentState<IronworksState>>,
) {
	let pending = *ironworks_state == CurrentState(IronworksState::ResourceRequested);

	egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
		ui.vertical_centered(|ui| {
			ui.heading("nero");

			// TODO: Work out how to show errors from path validation.
			ui.label("Could not find game installation path.");

			if ui
				.add_enabled(!pending, egui::Button::new("Select game folder"))
				.clicked()
			{
				commands.insert_resource(NextState(IronworksState::ResourceRequested));
			}
		})
	});
}

#[derive(Default)]
struct ToolState {
	icons: Option<Vec<(Handle<Image>, egui::TextureId)>>,
}

fn ui_toolbox(
	mut commands: Commands,
	mut egui_context: ResMut<EguiContext>,
	mut tool_state: Local<ToolState>,
	asset_server: Res<AssetServer>,
) {
	// Get the icons for tools, creating them if they don't exist yet.
	let tool_icons = tool_state.icons.get_or_insert_with(|| {
		Tool::iter()
			.map(|tool| {
				let handle = asset_server.load(tool.icon());
				let id = egui_context.add_image(handle.clone());
				(handle, id)
			})
			.collect()
	});

	let ctx = egui_context.ctx_mut();

	// Render the primary toolbox along the left side.
	egui::SidePanel::left("toolbox")
		.width_range(20.0..=20.0)
		.resizable(false)
		.frame(egui::Frame::default().fill(ctx.style().visuals.window_fill()))
		.show(ctx, |ui| {
			for (index, tool) in Tool::iter().enumerate() {
				let button = egui::ImageButton::new(tool_icons[index].1, [24.0, 24.0])
					.tint(ui.style().visuals.text_color());
				let response = ui.add(button).on_hover_text_at_pointer(tool.name());
				if response.clicked() {
					commands.insert_resource(NextState(tool))
				}
			}
		});
}
