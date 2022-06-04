use std::{borrow::Cow, fmt, io::Cursor};

use binrw::{BinRead, NullString};
use getset::{CopyGetters, Getters};

use crate::{error::Result, file::File};

use super::structs;

/// A material. Contains metadata to be used by shaders, and references to the
/// shader and requisite assets used by the material.
pub struct Material {
	file: structs::Material,
	shader: String,
	// TODO: Color sets
	// TODO: Shader keys
	// TODO: Constants
	// TODO: Shader values
	samplers: Vec<Sampler>,
}

// Public API surface.
impl Material {
	/// Version of the material structure.
	pub fn version(&self) -> u32 {
		self.file.version
	}

	/// Name of the shader file or package used by this material.
	pub fn shader(&self) -> &str {
		&self.shader
	}

	// ??????
	// todo what should this be called
	pub fn color_set(&self) -> Option<&[u8; 512]> {
		self.file.color_set_texture.as_ref()
	}

	/// Texture samplers used by the material.
	pub fn samplers(&self) -> &[Sampler] {
		&self.samplers
	}
}

// Construction logic.
impl Material {
	fn read_shader(file: &structs::Material) -> Result<String> {
		let mut cursor = Cursor::new(&file.string_data);
		cursor.set_position(file.shader_package_name_offset.into());
		let shader = NullString::read(&mut cursor)?.into_string();
		Ok(shader)
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
		Ok(Material {
			shader: Material::read_shader(&file)?,
			samplers: Material::read_samplers(&file)?,
			file,
		})
	}
}

impl fmt::Debug for Material {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Material")
			.field("version", &self.version())
			.field("shader", &self.shader)
			.field("samplers", &self.samplers)
			.finish()
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
