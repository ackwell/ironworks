use std::{io::Cursor, path::Path, sync::Arc};

use anyhow::Context;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use ironworks::{file::tex, Ironworks};
use itertools::Itertools;

use crate::{
	data::{self},
	version::VersionKey,
};

use super::error::{Error, Result};

pub struct Service {
	data: Arc<data::Data>,
}

impl Service {
	pub fn new(data: Arc<data::Data>) -> Self {
		Self { data }
	}

	pub fn convert(&self, version: VersionKey, path: &str, format: Format) -> Result<Vec<u8>> {
		let data_version = self
			.data
			.version(version)
			.with_context(|| format!("data for {version} not ready"))?;

		let converter = format.converter();
		converter.convert(&data_version, path, format)
	}
}

// todo: should probably put the fromstr on it here, but expose it so http can get the format and use it for mime which is http-specific
// TODO: proper tostring impl for this?
#[derive(Debug, Clone, Copy)]
pub enum Format {
	Png,
}

impl Format {
	fn converter(&self) -> &dyn Converter {
		match self {
			Self::Png => &ImageConverter,
		}
	}
}

trait Converter {
	// TODO: Consider using a stream for this - the only converter I actually have right now doesn't operate with streams, but it may be relevant for other converters - or possibly would tie in with caching. Ref. https://github.com/tokio-rs/axum/discussions/608 re: responding to requests with streams.
	fn convert(&self, data: &data::Version, path: &str, format: Format) -> Result<Vec<u8>>;
}

struct ImageConverter;

impl Converter for ImageConverter {
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

	// TODO: break this up into functions - might be able to reuse a bunch of stuff between basic rgba stuff
	let buffer = match texture.format() {
		// TODO: seems really wasteful to copy the entire image in memory just to reassign the channels. think of a better way to do this.
		// TODO: consider writing a chunk iterator that uses exact widths rather than moving into a tuple
		tex::Format::Argb8 => {
			let data = texture
				.data()
				.iter()
				.tuples()
				.flat_map(|(b, g, r, a)| [r, g, b, a])
				.copied()
				.collect::<Vec<_>>();

			let buffer =
				ImageBuffer::from_raw(texture.width().into(), texture.height().into(), data)
					.context("failed to build image buffer")?;
			DynamicImage::ImageRgba8(buffer)
		}

		other => {
			return Err(Error::UnsupportedSource(
				path.into(),
				format!("unhandled texture format {other:?}"),
			))
		}
	};

	Ok(buffer)
}
