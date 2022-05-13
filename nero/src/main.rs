use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::ironworks::{IronworksAssetIoPlugin, IronworksPlugin, List};

mod ironworks;

fn main() {
	App::new()
		.add_plugins_with(DefaultPlugins, |group| {
			group.add_before::<bevy::asset::AssetPlugin, _>(IronworksAssetIoPlugin)
		})
		// UI
		.add_plugin(EguiPlugin)
		.insert_resource(WinitSettings::desktop_app())
		.add_system(system_ui)
		// View
		.add_startup_system(startup_test)
		// Ironworks
		.add_plugin(IronworksPlugin)
		// Done
		.run();
}

struct TempTestRes {
	handle: Handle<List>,
}

fn startup_test(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
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
	commands.spawn_bundle(PbrBundle {
		// mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		mesh: asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_char1.mdl"),
		material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	// TODO: realistically this shouldn't be here. just using to test. should it be an entity?
	commands.insert_resource(TempTestRes {
		handle: asset_server.load("iw://exd/root.exl"),
	})
}

fn system_ui(
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
