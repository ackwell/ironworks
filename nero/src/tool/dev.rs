use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use super::Tool;

pub struct DevTool;
impl Plugin for DevTool {
	fn build(&self, app: &mut App) {
		app.add_enter_system(Some(Tool::Dev), asset_test)
			.add_exit_system(Some(Tool::Dev), cleanup);
	}
}

// TODO: this will probably be behavior I want on all tools - how should I share it?
#[derive(Component)]
struct TransientMarker;

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
	// let scene_handle: Handle<Scene> =
	// 	asset_server.load("iw://bg/ffxiv/sea_s1/twn/common/bgparts/s1t0_z0_flg3.mdl"))
	// let scene_handle: Handle<Scene> =
	// 	asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_char1.mdl");
	// let scene_handle: Handle<Scene> =
	// 	asset_server.load("iw://bg/ffxiv/sea_s1/twn/s1ta/bgparts/s1ta_ga_flr2.mdl");
	// let scene_handle: Handle<Scene> =
	// 	asset_server.load("iw://bg/ffxiv/wil_w1/dun/w1d5/bgparts/w1d5_q1_bre4b.mdl");
	let scene_handle: Handle<Scene> =
		asset_server.load("iw://bg/ffxiv/wil_w1/dun/w1d5/bgparts/w1d5_q1_bre4b.mdl");

	commands
		.spawn()
		.insert(TransientMarker)
		.with_children(|children| {
			children.spawn_scene(scene_handle);
		});

	// commands.spawn_bundle(PointLightBundle {
	// 	point_light: PointLight {
	// 		intensity: 1500.0,
	// 		shadows_enabled: true,
	// 		..default()
	// 	},
	// 	transform: Transform::from_xyz(4.0, 8.0, 4.0),
	// 	..default()
	// });
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<TransientMarker>>) {
	for entity in entities.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
