use std::{io::Cursor, iter};

use anyhow::Context;
use bevy::{
	asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
};
use ironworks::file::{sklb, File};
use mayhem::{tagfile, NodeWalker};

#[derive(Default)]
pub struct SklbAssetLoader;

impl AssetLoader for SklbAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move { load_sklb(bytes, load_context) })
	}

	fn extensions(&self) -> &[&str] {
		&["sklb"]
	}
}

fn load_sklb<'a>(
	bytes: &'a [u8],
	load_context: &'a mut LoadContext<'_>,
) -> Result<(), anyhow::Error> {
	// Read .sklb and extract the skeleton data
	let sklb = <sklb::SkeletonBinary>::read(bytes)?;
	let root_node = tagfile::read(&mut Cursor::new(sklb.skeleton()))?;
	let skeleton = get_skeleton(root_node).context("Failed to extract skeleton data.")?;

	// We model a skeleton as a tree of entities, set up a world for the asset
	// scene, and a store of IDs for parent references.
	let mut world = World::default();
	let mut entities = Vec::new();

	// Temporary mesh/material to visualise the entitiy tree.
	// TODO: remove these
	let mesh = load_context.set_labeled_asset(
		"DebugMesh",
		LoadedAsset::new(Mesh::from(shape::Box::new(0.02, 0.01, 0.01))),
	);
	let material = load_context.set_labeled_asset(
		"DebugMaterial",
		LoadedAsset::new(StandardMaterial::from(Color::rgb(1.0, 0.0, 0.0))),
	);

	for (parent, transform) in iter::zip(skeleton.parents.into_iter(), skeleton.pose.into_iter()) {
		// Spawn an entity for this bone.
		let entity = world
			.spawn()
			.insert_bundle(MaterialMeshBundle {
				mesh: mesh.clone(),
				material: material.clone(),
				transform: Transform {
					translation: Vec3::from_slice(&transform[0..3]),
					rotation: Quat::from_slice(&transform[4..8]),
					scale: Vec3::from_slice(&transform[8..11]),
				},
				..Default::default()
			})
			.id();

		// If this bone has a defined parent, add it as a child.
		if parent >= 0 {
			world
				.entity_mut(entities[usize::try_from(parent).unwrap()])
				.push_children(&[entity]);
		}

		// Add the new entitiy to the parent reference vec.
		entities.push(entity);
	}

	load_context.set_default_asset(LoadedAsset::new(Scene::new(world)));

	Ok(())
}

// TODO: I really need to improve the exposed deserialisation interface in mayhem, this is not pretty.
fn get_skeleton(root_node: NodeWalker) -> Option<Skeleton> {
	let named_variant = root_node
		.field("namedVariants")?
		.as_vector()?
		.iter()
		.find_map(|value| {
			let node = root_node.node(*value.as_node()?);
			match node.field("name")?.as_string()?.as_str() {
				"hkaAnimationContainer" => Some(node),
				_ => None,
			}
		})?;

	let animation_container = named_variant.node(*named_variant.field("variant")?.as_node()?);
	let skeleton = animation_container
		.node(*animation_container.field("skeletons")?.as_vector()?[0].as_node()?);

	Some(Skeleton {
		parents: skeleton.field("parentIndices")?.try_into().ok()?,
		pose: skeleton.field("referencePose")?.try_into().ok()?,
	})
}

// TODO: should probably expose, like iter_bone which combos? or is that not nessecary?
struct Skeleton {
	parents: Vec<i32>,
	// TODO: actually Vec<[f32; 12]>, need to work out how to handle conversion like that.
	pose: Vec<Vec<f32>>,
}
