use std::{borrow::Cow, collections::HashSet, path::PathBuf};

use bevy::{
	asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	render::{
		mesh::{Indices, MeshVertexAttribute, PrimitiveTopology},
		render_resource::VertexFormat,
	},
};
use ironworks::file::{mdl, File};

use crate::render::{Material, MeshBundle, ATTRIBUTE_COLOR, ATTRIBUTE_UV_4};

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
	// TODO: load all 3 LOD as seperate scenes? I'll also need to load variants, and there's potentially >100 of those for some - so loading them all eagerly seems like a bad idea. Might make sense to register own asset type for these, and let consumer code specify lod/variant/etc?
	let model = container.model(mdl::Lod::High);
	let meshes = model.meshes().into_iter().map(load_mesh);

	let mut dependencies = HashSet::<String>::new();

	for (index, result) in meshes.enumerate() {
		// TODO: not super happy about the delayed result handling on this
		let (mesh, mtrl_path) = result?;

		let mesh_handle =
			load_context.set_labeled_asset(&format!("Mesh{index}"), LoadedAsset::new(mesh));

		let material = load_context.get_handle::<_, Material>(&mtrl_path);
		dependencies.insert(mtrl_path);

		world.spawn().insert_bundle(MeshBundle {
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

	// TODO: work out where this should go
	const MEME: MeshVertexAttribute =
		MeshVertexAttribute::new("Vertex_Color", 100, VertexFormat::Float32x4);

	for mdl::VertexAttribute { kind, values } in vertex_attributes {
		use mdl::VertexAttributeKind as K;
		match kind {
			K::Position => mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, to_f32x3(values)),
			K::Normal => mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, to_f32x3(values)),
			K::Uv => mesh.insert_attribute(ATTRIBUTE_UV_4, to_f32x4(values)),
			K::Color => mesh.insert_attribute(ATTRIBUTE_COLOR, to_f32x4(values)),
			other => info!("TODO: {other:?}"),
		};
	}

	// If the mesh doesn't define color, fill it in with a default.
	// TODO: This is a bit silly, but saves needing to reimplement the mesh pipeline to add shader definitions to enable the color handling per-mesh. When shader requirements scale up and that's on the table, revisit this.
	if !mesh.contains_attribute(MEME) {
		mesh.insert_attribute(MEME, vec![[1.0, 1.0, 1.0, 0.0]; mesh.count_vertices()]);
	}

	mesh.set_indices(Some(Indices::U16(indices)));

	// TODO: is this the "right" place for the iw prefix?
	Ok((
		mesh,
		format!("iw://{}", resolve_material(&mdl_mesh.material()?)),
	))
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

fn to_f32x4(values: mdl::VertexValues) -> Vec<[f32; 4]> {
	use mdl::VertexValues as V;
	match values {
		V::Vector2(vec) => vec.into_iter().map(|[x, y]| [x, y, 0., 0.]).collect(),
		V::Vector3(vec) => vec.into_iter().map(|[x, y, z]| [x, y, z, 0.]).collect(),
		V::Vector4(vec) => vec,
		other => panic!("Cannot convert {other:?} to f32x4."),
	}
}

fn resolve_material(path: &str) -> Cow<str> {
	// TODO: this is common logic across all xiv related stuff - it should probably be part of some IW feature/module

	if !path.starts_with('/') {
		return path.into();
	}

	// TODO: work out how to wire variant - this'll be pretty important lmao. Data in .imc?
	let variant = 1;
	let value1 = &path[5..9];
	let value2 = &path[10..14];

	match (&path[4..5], &path[9..10]) {
		("c", "b") => {
			format!("chara/human/c{value1}/obj/body/b{value2}/material/v{variant:04}{path}")
		}
		("c", "e") => format!("chara/equipment/e{value2}/material/v{variant:04}{path}"),
		other => todo!("Unknown pair {other:?} in path {path}."),
	}
	.into()
}
