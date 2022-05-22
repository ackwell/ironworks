use std::{collections::HashSet, path::PathBuf};

use bevy::{
	asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	render::mesh::{Indices, PrimitiveTopology},
};
use ironworks::file::{mdl, File};

use crate::material::BgMaterial;

#[derive(Default)]
pub struct MdlAssetLoader;

impl AssetLoader for MdlAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
		Box::pin(async move { load_mdl(bytes, load_context) })
	}

	fn extensions(&self) -> &[&str] {
		&["mdl"]
	}
}

fn load_mdl<'a>(
	bytes: &'a [u8],
	load_context: &'a mut LoadContext<'_>,
) -> Result<(), anyhow::Error> {
	let mut world = World::default();

	let container = <mdl::ModelContainer as File>::read(bytes)?;
	// TODO: load all 3 as seperate scenes?
	let model = container.model(mdl::Lod::High);
	let meshes = model.meshes().into_iter().map(load_mesh);

	let mut dependencies = HashSet::<String>::new();

	for (index, result) in meshes.enumerate() {
		// TODO: not super happy about the delayed result handling on this
		let (mesh, mtrl_path) = result?;

		let mesh_handle =
			load_context.set_labeled_asset(&format!("Mesh{index}"), LoadedAsset::new(mesh));

		// TODO: There's >1 material type that i'll need to use eod - i guess make them an enum, or something? that or focus on reading xiv shpk next.
		let material = load_context.get_handle::<_, BgMaterial>(&mtrl_path);
		dependencies.insert(mtrl_path);

		// TODO: might want own bundle type for this?
		world.spawn().insert_bundle(MaterialMeshBundle {
			mesh: mesh_handle,
			material,
			..Default::default()
		});
	}

	let scene = Scene::new(world);

	let dependency_array = dependencies
		.into_iter()
		.map(|path| AssetPath::from(PathBuf::from(path)))
		.collect::<Vec<_>>();
	load_context.set_default_asset(LoadedAsset::new(scene).with_dependencies(dependency_array));

	Ok(())
}

// todo: use a struct for the return type if it's anything more than whats there
fn load_mesh(mdl_mesh: mdl::Mesh) -> Result<(Mesh, String), ironworks::Error> {
	let indices = mdl_mesh.indices()?;
	let vertex_attributes = mdl_mesh.attributes()?;

	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

	for mdl::VertexAttribute { kind, values } in vertex_attributes {
		use mdl::VertexAttributeKind as K;
		match kind {
			K::Position => mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, to_f32x3(values)),
			K::Normal => mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, to_f32x3(values)),
			K::Uv => mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, to_f32x2(values)),
			other => info!("TODO: {other:?}"),
		};
	}

	mesh.set_indices(Some(Indices::U16(indices)));

	// TODO: is this the "right" place for the iw prefix?
	Ok((mesh, format!("iw://{}", mdl_mesh.material()?)))
}

fn to_f32x2(values: mdl::VertexValues) -> Vec<[f32; 2]> {
	use mdl::VertexValues as V;
	match values {
		V::Vector2(vec) => vec,
		V::Vector3(vec) => vec.into_iter().map(|[x, y, _z]| [x, y]).collect(),
		V::Vector4(vec) => vec.into_iter().map(|[x, y, _z, _w]| [x, y]).collect(),
		other => panic!("Cannot convert {other:?} to f32x3."),
	}
}

fn to_f32x3(values: mdl::VertexValues) -> Vec<[f32; 3]> {
	use mdl::VertexValues as V;
	match values {
		V::Vector2(vec) => vec.into_iter().map(|[x, y]| [x, y, 0.]).collect(),
		V::Vector3(vec) => vec,
		V::Vector4(vec) => vec.into_iter().map(|[x, y, z, _w]| [x, y, z]).collect(),
		other => panic!("Cannot convert {other:?} to f32x3."),
	}
}
