use std::{borrow::Cow, io::Cursor};

use binrw::{BinRead, NullString};
use getset::{CopyGetters, Getters};

use crate::{error::Result, file::File};

use super::structs;

/// A material. Contains metadata to be used by shaders, and references to the
/// shader and requisite assets used by the material.
#[derive(Debug)]
pub struct Material {
	file: structs::Material,
	samplers: Vec<Sampler>,
}

impl Material {
	/// Texture samplers used by the material.
	pub fn samplers(&self) -> &Vec<Sampler> {
		&self.samplers
	}

	fn read_samplers(file: &structs::Material) -> Result<Vec<Sampler>> {
		let mut cursor = Cursor::new(&file.string_data);

		file.samplers
			.iter()
			.map(|sampler| {
				let offset = &file.texture_offsets[usize::from(sampler.texture_index)];
				cursor.set_position(offset.offset.into());
				let texture = NullString::read(&mut cursor)?.into_string();

				Ok(Sampler {
					id: sampler.id,
					// state: sampler.state,
					texture,
				})
			})
			.collect()
	}
}

impl File for Material {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		let file = structs::Material::read(&mut Cursor::new(data.into()))?;
		let samplers = Material::read_samplers(&file)?;
		Ok(Material { file, samplers })
	}
}

/// Texture sampler for a material.
#[derive(Debug, Getters, CopyGetters)]
pub struct Sampler {
	/// Sampler ID.
	#[get_copy = "pub"]
	id: u32,
	// state: u32,
	texture: String,
}

impl Sampler {
	/// Path to the texture used by this sampler. Path is not guaranteed to be absolute.
	pub fn texture(&self) -> String {
		self.texture.clone()
	}
}
