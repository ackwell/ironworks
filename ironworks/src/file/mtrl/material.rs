// TODO: remove
#![allow(missing_docs, dead_code)]

use std::{borrow::Cow, io::Cursor};

use binrw::{BinRead, NullString};

use crate::{error::Result, file::File};

use super::structs;

#[derive(Debug)]
pub struct Material {
	file: structs::Material,
}

impl Material {
	// TODO: iterator?
	pub fn samplers(&self) -> Result<Vec<Sampler>> {
		let file = &self.file;

		let mut cursor = Cursor::new(&file.string_data);

		file.samplers
			.iter()
			.map(|sampler| {
				let offset = &file.texture_offsets[usize::from(sampler.texture_index)];
				cursor.set_position(offset.offset.into());
				let texture = NullString::read(&mut cursor)?.into_string();

				Ok(Sampler {
					id: sampler.id,
					state: sampler.state,
					texture,
				})
			})
			.collect()
	}
}

impl File for Material {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		let file = structs::Material::read(&mut Cursor::new(data.into()))?;
		Ok(Material { file })
	}
}

// todo: can i do this eagerly?
#[derive(Debug)]
pub struct Sampler {
	id: u32,
	// TODO: bitfield
	state: u32,

	texture: String,
}
