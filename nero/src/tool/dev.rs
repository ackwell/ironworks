use std::io::Cursor;

use bevy::{
	prelude::{shape::Box, *},
	utils::HashMap,
};
use ironworks::file::sklb;
use iyes_loopless::prelude::AppLooplessStateExt;
use mayhem::tagfile;

use crate::asset_io::IronworksResource;

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
	// asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	ironworks: Res<IronworksResource>,
) {
	// 2D texture test
	// commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	// commands.spawn_bundle(SpriteBundle {
	// 	texture: asset_server
	// 		.load("iw://chara/human/c0201/obj/face/f0002/texture/--c0201f0002_iri_n.tex"),
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
	// let scene_handle: Handle<Scene> =
	// 	asset_server.load("iw://bg/ffxiv/wil_w1/dun/w1d5/bgparts/w1d5_q1_bre4b.mdl");

	// commands
	// 	.spawn()
	// 	.insert(TransientMarker)
	// 	.with_children(|children| {
	// 		children.spawn_scene(scene_handle);
	// 	});

	// commands.spawn_bundle(PointLightBundle {
	// 	point_light: PointLight {
	// 		intensity: 1500.0,
	// 		shadows_enabled: true,
	// 		..default()
	// 	},
	// 	transform: Transform::from_xyz(4.0, 8.0, 4.0),
	// 	..default()
	// });

	let sklb = ironworks
		.read()
		.unwrap()
		.file::<sklb::SkeletonBinary>("chara/human/c1801/skeleton/base/b0001/skl_c1801b0001.sklb")
		.unwrap();

	let root_node = tagfile::read(&mut Cursor::new(sklb.skeleton())).unwrap();

	let named_variant = root_node
		.field("namedVariants")
		.unwrap()
		.as_vector()
		.unwrap()
		.iter()
		.find_map(|value| {
			let node = root_node.node(*value.as_node().unwrap());
			match node.field("name").unwrap().as_string().unwrap().as_str() {
				"hkaAnimationContainer" => Some(node),
				_ => None,
			}
		})
		.unwrap();
	let animation_container =
		named_variant.node(*named_variant.field("variant").unwrap().as_node().unwrap());
	let skeleton = animation_container.node(
		*animation_container
			.field("skeletons")
			.unwrap()
			.as_vector()
			.unwrap()[0]
			.as_node()
			.unwrap(),
	);

	let parents = Vec::<i32>::try_from(skeleton.field("parentIndices").unwrap()).unwrap();
	let pose = Vec::<Vec<f32>>::try_from(skeleton.field("referencePose").unwrap()).unwrap();

	let pose_iter = std::iter::zip(parents.into_iter(), pose.into_iter());

	let mesh = meshes.add(Mesh::from(Box::new(0.02, 0.01, 0.01)));
	let material = materials.add(Color::rgb(1.0, 0.0, 0.0).into());

	// TODO: probably need to do this in reverse or something

	let mut root_entity = commands.spawn();
	root_entity
		.insert(TransientMarker)
		.insert_bundle(MaterialMeshBundle {
			mesh: meshes.add(Mesh::from(Box::new(0.02, 0.02, 0.02))),
			material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
			..Default::default()
		});
	let mut tree = HashMap::new();
	tree.insert(-1, root_entity.id());

	for (index, (parent, pos)) in pose_iter.enumerate() {
		let entity = commands
			.spawn_bundle(MaterialMeshBundle {
				mesh: mesh.clone(),
				material: material.clone(),
				transform: Transform {
					translation: Vec3::from_slice(&pos[0..3]),
					rotation: Quat::from_slice(&pos[4..8]),
					..Default::default()
				},
				..Default::default()
			})
			.id();
		commands
			.entity(*tree.get(&parent).unwrap())
			.add_child(entity);
		tree.insert(index.try_into().unwrap(), entity);
	}

	commands.insert_resource(AmbientLight {
		brightness: 1.0,
		..Default::default()
	})
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<TransientMarker>>) {
	for entity in entities.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
