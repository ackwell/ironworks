use std::{io::Cursor, path::Path};

use anyhow::Context;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use ironworks::{file::tex, Ironworks};
use itertools::Itertools;

use crate::data;

use super::{
	error::{Error, Result},
	format::Format,
};

pub trait Converter {
	// TODO: Consider using a stream for this - the only converter I actually have right now doesn't operate with streams, but it may be relevant for other converters - or possibly would tie in with caching. Ref. https://github.com/tokio-rs/axum/discussions/608 re: responding to requests with streams.
	fn convert(&self, data: &data::Version, path: &str, format: Format) -> Result<Vec<u8>>;
}

pub struct Image;

impl Converter for Image {
	fn convert(&self, data: &data::Version, path: &str, format: Format) -> Result<Vec<u8>> {
		let extension = Path::new(path)
			.extension()
			.and_then(|extension| extension.to_str());

		// TODO: add error handling case on this once more than one format exists.
		let output_format = match format {
			Format::Png => ImageOutputFormat::Png,
		};

		// TODO: should i just pass IW to convert? is there any realistic expectation that a converter will need excel?
		let ironworks = data.ironworks();

		let buffer = match extension {
			Some("tex") => read_texture(&ironworks, path),

			other => {
				return Err(Error::InvalidConversion(
					other.unwrap_or("(none)").into(),
					format,
				));
			}
		}?;

		// TODO: are there any non-failure cases here?
		let mut bytes = Cursor::new(vec![]);
		buffer
			.write_to(&mut bytes, output_format)
			.context("failed to write output buffer")?;

		Ok(bytes.into_inner())
	}
}

fn read_texture(ironworks: &Ironworks, path: &str) -> Result<DynamicImage> {
	let texture = match ironworks.file::<tex::Texture>(path) {
		Ok(value) => value,
		Err(ironworks::Error::NotFound(_)) => return Err(Error::NotFound(path.into())),
		other => other.context("read file")?,
	};

	if texture.dimension() != tex::Dimension::D2 {
		return Err(Error::UnsupportedSource(
			path.into(),
			format!("unhandled texture dimension {:?}", texture.dimension()),
		));
	}

	let buffer = match texture.format() {
		tex::Format::Argb8 => read_texture_argb8(texture)?,

		tex::Format::Dxt1 => read_texture_dxt(texture, texpresso::Format::Bc1)?,
		tex::Format::Dxt3 => read_texture_dxt(texture, texpresso::Format::Bc2)?,
		tex::Format::Dxt5 => read_texture_dxt(texture, texpresso::Format::Bc3)?,

		other => {
			return Err(Error::UnsupportedSource(
				path.into(),
				format!("unhandled texture format {other:?}"),
			))
		}
	};

	Ok(buffer)
}

fn read_texture_argb8(texture: tex::Texture) -> Result<DynamicImage> {
	// TODO: seems really wasteful to copy the entire image in memory just to reassign the channels. think of a better way to do this.
	// TODO: consider writing a chunk iterator that uses exact widths rather than moving into a tuple
	let data = texture
		.data()
		.iter()
		.tuples()
		.flat_map(|(b, g, r, a)| [r, g, b, a])
		.copied()
		.collect::<Vec<_>>();

	let buffer = ImageBuffer::from_raw(texture.width().into(), texture.height().into(), data)
		.context("failed to build image buffer")?;
	Ok(DynamicImage::ImageRgba8(buffer))
}

fn read_texture_dxt(texture: tex::Texture, dxt_format: texpresso::Format) -> Result<DynamicImage> {
	let width = usize::from(texture.width());
	let height = usize::from(texture.height());

	let mut dxt_buffer = vec![0; width * height * 4];
	dxt_format.decompress(texture.data(), width, height, &mut dxt_buffer);

	let image_buffer = ImageBuffer::from_raw(
		width.try_into().unwrap(),
		height.try_into().unwrap(),
		dxt_buffer,
	)
	.context("failed to build image buffer")?;
	Ok(DynamicImage::ImageRgba8(image_buffer))
}
