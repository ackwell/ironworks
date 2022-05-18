use bevy::{
	asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	render::mesh::{Indices, PrimitiveTopology},
};
use ironworks::file::{mdl, File};

#[derive(Default)]
pub struct MdlAssetLoader;

impl AssetLoader for MdlAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
		Box::pin(async move { Ok(load_mdl(bytes, load_context)?) })
	}

	fn extensions(&self) -> &[&str] {
		&["mdl"]
	}
}

// todo: mdls contain more than a single mesh, need to take a page out of i.e. gltf loader for this eventually
fn load_mdl<'a>(
	bytes: &'a [u8],
	load_context: &'a mut LoadContext,
) -> Result<(), ironworks::Error> {
	let mut world = World::default();

	let container = <mdl::ModelContainer as File>::read(bytes)?;
	// TODO: load all 3 as seperate scenes?
	let model = container.model(mdl::Lod::High);
	let meshes = model.meshes().into_iter().map(load_mesh);

	// TODO: mtrl
	load_context.set_labeled_asset(
		"TEMPMATERIAL",
		LoadedAsset::new(StandardMaterial::from(Color::rgb(1., 1., 1.))),
	);

	for (index, mesh) in meshes.enumerate() {
		// TODO: not super happy about the delayed result handling on this
		let key = &format!("Mesh{}", index);
		load_context.set_labeled_asset(key, LoadedAsset::new(mesh?));

		// TODO: might want own bundle type for this?
		world.spawn().insert_bundle(PbrBundle {
			mesh: load_context.get_handle(AssetPath::new_ref(load_context.path(), Some(key))),
			material: load_context.get_handle(AssetPath::new_ref(
				load_context.path(),
				Some("TEMPMATERIAL"),
			)),
			..Default::default()
		});
	}

	let scene = Scene::new(world);

	load_context.set_default_asset(LoadedAsset::new(scene));

	Ok(())
}

fn load_mesh(mdl_mesh: mdl::Mesh) -> Result<Mesh, ironworks::Error> {
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

	Ok(mesh)
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
