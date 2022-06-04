use bevy::{
	asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
	prelude::*,
	render::render_resource::{
		Extent3d, FilterMode, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat,
	},
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
	let (format, data) = match tex.format() {
		tex::Format::Rgb5a1 => convert_rgb5a1(&tex),
		tex::Format::Argb8 => convert_argb8(&tex),
		tex::Format::Dxt1 => convert_dxt1(&tex),
		tex::Format::Dxt3 => convert_dxt3(&tex),
		tex::Format::Dxt5 => convert_dxt5(&tex),
		other => todo!("Texture format: {other:?}"),
	};

	let mut image = Image::default();
	image.data = data;

	image.texture_descriptor = TextureDescriptor {
		size: Extent3d {
			width: tex.width().into(),
			height: tex.height().into(),
			depth_or_array_layers: tex.depth().into(),
		},
		mip_level_count: tex.mip_levels().into(),
		format,
		dimension: match tex.dimension() {
			tex::Dimension::D1 => TextureDimension::D1,
			tex::Dimension::D2 => TextureDimension::D2,
			tex::Dimension::D3 => TextureDimension::D3,
			other => todo!("Texture dimension: {other:?}"),
		},
		..image.texture_descriptor
	};

	// TODO: xiv textures don't bundle sampler info, work out how to derive this
	// TODO: work out how to configure mipmap usage, this gets really blurry really quickly
	image.sampler_descriptor = SamplerDescriptor {
		mag_filter: FilterMode::Linear,
		min_filter: FilterMode::Linear,
		mipmap_filter: FilterMode::Linear,
		..image.sampler_descriptor
	};

	image
}

fn convert_rgb5a1(tex: &tex::Texture) -> (TextureFormat, Vec<u8>) {
	// TODO: this is jank. improve.
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

	(TextureFormat::Rgba8UnormSrgb, converted)
}

fn convert_argb8(tex: &tex::Texture) -> (TextureFormat, Vec<u8>) {
	let data = tex
		.data()
		.chunks_exact(4)
		.flat_map(|chunk| {
			let (a, r, g, b) = (chunk[0], chunk[1], chunk[2], chunk[3]);
			[r, g, b, a]
		})
		.collect::<Vec<_>>();

	(TextureFormat::Rgba8UnormSrgb, data)
}

fn convert_dxt1(tex: &tex::Texture) -> (TextureFormat, Vec<u8>) {
	(TextureFormat::Bc1RgbaUnormSrgb, tex.data().to_vec())
}

fn convert_dxt3(tex: &tex::Texture) -> (TextureFormat, Vec<u8>) {
	(TextureFormat::Bc2RgbaUnormSrgb, tex.data().to_vec())
}

fn convert_dxt5(tex: &tex::Texture) -> (TextureFormat, Vec<u8>) {
	(TextureFormat::Bc3RgbaUnormSrgb, tex.data().to_vec())
}
