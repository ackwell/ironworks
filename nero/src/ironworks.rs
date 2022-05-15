use std::{
	io,
	path::{Path, PathBuf},
	sync::{Arc, RwLock},
};

use bevy::{
	asset::{AssetIo, AssetIoError, AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	reflect::TypeUuid,
	render::{
		mesh::{Indices, PrimitiveTopology},
		render_resource::{Extent3d, TextureDimension, TextureFormat},
	},
};
use ironworks::{
	ffxiv,
	file::{exl, mdl, tex, File},
	sqpack::SqPack,
	Error, ErrorValue, Ironworks,
};

pub struct IronworksAssetIoPlugin;

impl Plugin for IronworksAssetIoPlugin {
	fn build(&self, app: &mut App) {
		let task_pool = app
			.world
			.get_resource::<bevy::tasks::IoTaskPool>()
			.expect("IoTaskPool resource not found")
			.0
			.clone();

		let default_io = bevy::asset::create_platform_default_asset_io(app);

		let ironworks = Arc::new(RwLock::new(Ironworks::new()));

		// TODO: Try this eagerly with search and otherwise defer to adding resource later with explicit path?
		ironworks
			.write()
			.unwrap()
			.add_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));

		let asset_io = IronworksAssetIo {
			default_io,
			ironworks,
		};

		app.insert_resource(AssetServer::new(asset_io, task_pool));
	}
}

pub struct IronworksPlugin;

impl Plugin for IronworksPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<ListAssetLoader>()
			.init_asset_loader::<MdlAssetLoader>()
			.init_asset_loader::<TexAssetLoader>()
			.add_asset::<List>();
	}
}

struct IronworksAssetIo {
	default_io: Box<dyn AssetIo>,

	ironworks: Arc<RwLock<Ironworks>>,
}

impl AssetIo for IronworksAssetIo {
	fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
		if let Ok(ironworks_path) = path.strip_prefix("iw://") {
			Box::pin(async move {
				self.ironworks
					.read()
					.unwrap()
					.file::<Vec<u8>>(&ironworks_path.to_string_lossy())
					.map_err(|error| match error {
						Error::NotFound(ErrorValue::Path(path)) => {
							AssetIoError::NotFound(path.into())
						}
						other => AssetIoError::Io(io::Error::new(io::ErrorKind::Other, other)),
					})
			})
		} else {
			self.default_io.load_path(path)
		}
	}

	// The below just pass through to the base asset io, is it worth handling dirs, or changes for penumbra style resources?

	fn read_directory(
		&self,
		path: &std::path::Path,
	) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
		self.default_io.read_directory(path)
	}

	fn is_directory(&self, path: &Path) -> bool {
		self.default_io.is_directory(path)
	}

	fn watch_path_for_changes(&self, path: &Path) -> Result<(), AssetIoError> {
		self.default_io.watch_path_for_changes(path)
	}

	fn watch_for_changes(&self) -> Result<(), AssetIoError> {
		self.default_io.watch_for_changes()
	}
}

// ???
// TODO: i'll need to newtype most iw stuff for asset handling, should i deref them?
#[derive(Debug, TypeUuid)]
#[uuid = "3584bf2d-97c2-42a1-a2a2-858f8bc4840b"]
pub struct List(pub exl::ExcelList);

// ??? temp stuff to test it works
#[derive(Default)]
struct ListAssetLoader;

impl AssetLoader for ListAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move {
			// TODO: this is pretty wasteful - none of the readers except vecu8 need an owned copy. that said, i'm not sure how best to handle the vec case - do i blindly copy and lose the passthrough benefit for other consumers? am i able to abuse asref or into<cow to allow [u8]|vecu80>vecu8?
			let list = exl::ExcelList::read(bytes.to_vec())?;

			load_context.set_default_asset(LoadedAsset::new(List(list)));

			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["exl"]
	}
}

#[derive(Default)]
struct MdlAssetLoader;

impl AssetLoader for MdlAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
		Box::pin(async move {
			let mdl = <mdl::ModelContainer as File>::read(bytes.to_vec())?;
			let mesh = convert_mdl(mdl);
			load_context.set_default_asset(LoadedAsset::new(mesh));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["mdl"]
	}
}

// todo: mdls contain more than a single mesh, need to take a page out of i.e. gltf loader for this eventually
fn convert_mdl(mdl: mdl::ModelContainer) -> Mesh {
	// todo:just pulling a single mesh out for now
	let mesh = mdl.lod(mdl::Lod::High).mesh(0);
	let indices = mesh.indices().unwrap();
	let vertex_attributes = mesh.vertices().unwrap();

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

	mesh.duplicate_vertices();
	mesh.compute_flat_normals();

	mesh
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

#[derive(Default)]
struct TexAssetLoader;

impl AssetLoader for TexAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move {
			let tex = <tex::Texture as File>::read(bytes.to_vec())?;
			let image = convert_tex(tex);
			load_context.set_default_asset(LoadedAsset::new(image));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["tex"]
	}
}

fn convert_tex(tex: tex::Texture) -> Image {
	match tex.format() {
		tex::Format::Rgb5a1 => convert_rgb5a1(tex),
		tex::Format::Dxt1 => convert_dxt1(tex),
		other => todo!("Texture format: {other:?}"),
	}
}

fn convert_rgb5a1(tex: tex::Texture) -> Image {
	// this is jank. improve.
	let data = tex.data();
	let converted = (0..data.len() / 2)
		.flat_map(|index| {
			let value = u16::from(data[index * 2]) + (u16::from(data[(index * 2) + 1]) << 8);

			[
				((value & 0x7C00) >> 7).try_into().unwrap(),
				((value & 0x03E0) >> 2).try_into().unwrap(),
				((value & 0x001F) << 3).try_into().unwrap(),
				(((value & 0x8000) >> 15) * 0xFF).try_into().unwrap(),
			]
		})
		.collect::<Vec<_>>();

	// TODO: flags in tex might have some extra info for this, like dimension
	Image::new(
		Extent3d {
			width: tex.width().into(),
			height: tex.height().into(),
			depth_or_array_layers: tex.depth().into(),
		},
		TextureDimension::D2,
		converted,
		TextureFormat::Rgba8UnormSrgb,
	)
}

fn convert_dxt1(tex: tex::Texture) -> Image {
	let width = tex.width();
	let height = tex.height();

	let mut decompressed = vec![0u8; 4 * usize::from(width) * usize::from(height)];

	squish::Format::Bc1.decompress(tex.data(), width.into(), height.into(), &mut decompressed);

	Image::new(
		Extent3d {
			width: width.into(),
			height: height.into(),
			depth_or_array_layers: tex.depth().into(),
		},
		TextureDimension::D2,
		decompressed,
		TextureFormat::Rgba8UnormSrgb,
	)
}
