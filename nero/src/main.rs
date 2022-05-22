#![allow(clippy::module_inception)]

mod asset_io;
mod asset_loaders;

use asset_io::{IronworksAssetIoPlugin, IronworksState};
use asset_loaders::{IronworksPlugin, List};
use bevy::{
	input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
	prelude::*,
	winit::WinitSettings,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;
use smooth_bevy_cameras::{
	controllers::orbit::{
		ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
	},
	LookTransformPlugin,
};

fn main() {
	App::new()
		// Ironworks
		.add_plugins_with(DefaultPlugins, |group| {
			group.add_before::<bevy::asset::AssetPlugin, _>(IronworksAssetIoPlugin)
		})
		.add_plugin(IronworksPlugin)
		// UI
		.add_plugin(EguiPlugin)
		.insert_resource(WinitSettings::desktop_app())
		.add_system(ui_need_ironworks_resource.run_not_in_state(IronworksState::Ready))
		.add_system(ui_main.run_in_state(IronworksState::Ready))
		// 3D
		.add_plugin(LookTransformPlugin)
		.add_plugin(OrbitCameraPlugin::new(true))
		.add_system(camera_controls)
		// Asset test stuff
		.add_enter_system(IronworksState::Ready, asset_test)
		// Done
		.run();
}

// Slightly tweaked copy of the default controls from the library because I didn't like the control scheme.
fn camera_controls(
	mut control_events: EventWriter<ControlEvent>,
	mut mouse_wheel_reader: EventReader<MouseWheel>,
	mut mouse_motion_events: EventReader<MouseMotion>,
	mouse_buttons: Res<Input<MouseButton>>,
	controllers: Query<&OrbitCameraController>,
) {
	// Get the controller for the camera.
	let controller = match controllers.iter().find(|controller| controller.enabled) {
		Some(controller) => controller,
		_ => return,
	};

	let OrbitCameraController {
		mouse_rotate_sensitivity,
		mouse_translate_sensitivity,
		mouse_wheel_zoom_sensitivity,
		pixels_per_line,
		..
	} = *controller;

	// Build the full mouse movement delta.
	let cursor_delta = mouse_motion_events
		.iter()
		.fold(Vec2::ZERO, |total, event| total + event.delta);

	// LMB translates on current plane.
	if mouse_buttons.pressed(MouseButton::Left) {
		control_events.send(ControlEvent::TranslateTarget(
			mouse_translate_sensitivity * cursor_delta,
		));
	}

	// RMB orbits current target.
	if mouse_buttons.pressed(MouseButton::Right) {
		control_events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
	}

	// Mouse wheel zooms current target.
	let zoom = mouse_wheel_reader.iter().fold(1.0, |total, event| {
		let amount = match event.unit {
			MouseScrollUnit::Line => event.y,
			MouseScrollUnit::Pixel => event.y * pixels_per_line,
		};
		total * (1.0 - amount * mouse_wheel_zoom_sensitivity)
	});
	control_events.send(ControlEvent::Zoom(zoom));
}

struct TempTestRes {
	handle: Handle<List>,
}

fn asset_test(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// mut meshes: ResMut<Assets<Mesh>>,
	// mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// // 2D texture test
	// commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	// commands.spawn_bundle(SpriteBundle {
	// 	texture: asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/texture/s1t0_a0_flag1_d.tex"),
	// 	..default()
	// });

	// 3D model test
	// commands.spawn_bundle(PbrBundle {
	// 	mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
	// 	material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
	// 	transform: Transform::from_xyz(0.0, 0.5, 0.0),
	// 	..default()
	// });
	commands
		.spawn_scene(asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/bgparts/s1t0_z0_flg3.mdl"));
	// commands.spawn_bundle(PbrBundle {
	// 	// mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
	// 	mesh: asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_char1.mdl"),
	// 	material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
	// 	..default()
	// });
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});

	commands.spawn_bundle(OrbitCameraBundle::new(
		OrbitCameraController::default(),
		PerspectiveCameraBundle::default(),
		Vec3::new(2.0, 0.0, 8.0),
		Vec3::ZERO,
	));

	// TODO: realistically this shouldn't be here. just using to test. should it be an entity?
	commands.insert_resource(TempTestRes {
		handle: asset_server.load("iw://exd/root.exl"),
	})
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

fn ui_main(
	mut egui_context: ResMut<EguiContext>,
	temp_test: Res<TempTestRes>,
	lists: Res<Assets<List>>,
) {
	// todo: better id
	egui::SidePanel::left("main")
		.resizable(true)
		.show(egui_context.ctx_mut(), |ui| {
			ui.heading("nero");

			if let Some(List(list)) = lists.get(&temp_test.handle) {
				let text = list.iter().fold(String::new(), |mut acc, cur| {
					acc.reserve(cur.len() + 1);
					acc.push_str(&cur);
					acc.push('\n');
					acc
				});
				ui.label(text);
			}
		});
}
