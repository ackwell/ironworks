use bevy::{
	asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use ironworks::file::{tex, File};

#[derive(Default)]
pub struct TexAssetLoader;

impl AssetLoader for TexAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move {
			let tex = <tex::Texture as File>::read(bytes)?;
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
