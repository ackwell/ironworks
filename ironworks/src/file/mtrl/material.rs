use std::{fmt, io::Cursor};

use binrw::{BinRead, NullString};
use getset::CopyGetters;

use crate::{FileStream, error::Result, file::File};

use super::structs;

/// A material: the shader a surface is drawn with, the textures bound to it, and the parameters
/// handed to that shader.
pub struct Material {
	version: u32,
	shader: String,
	shader_flags: u32,
	textures: Vec<Texture>,
	uv_sets: Vec<AttributeSet>,
	color_sets: Vec<AttributeSet>,
	samplers: Vec<Sampler>,
	shader_keys: Vec<ShaderKey>,
	constants: Vec<Constant>,
	shader_values: Vec<f32>,
	additional_data: Vec<u8>,
	color_table: Option<ColorTable>,
}

// Public API surface.
impl Material {
	/// Version of the material structure.
	pub fn version(&self) -> u32 {
		self.version
	}

	/// Name of the shader package used by this material.
	pub fn shader(&self) -> &str {
		&self.shader
	}

	/// Flags handed to the shader package.
	pub fn shader_flags(&self) -> u32 {
		self.shader_flags
	}

	/// Every texture the material references, in file order. Samplers index into this.
	pub fn textures(&self) -> &[Texture] {
		&self.textures
	}

	/// Named UV sets the shader can address.
	pub fn uv_sets(&self) -> &[AttributeSet] {
		&self.uv_sets
	}

	/// Named colour sets the shader can address.
	pub fn color_sets(&self) -> &[AttributeSet] {
		&self.color_sets
	}

	/// Texture samplers used by the material.
	pub fn samplers(&self) -> &[Sampler] {
		&self.samplers
	}

	/// Shader keys, which select variants within the shader package.
	pub fn shader_keys(&self) -> &[ShaderKey] {
		&self.shader_keys
	}

	/// Constants, each naming a span of [`shader_values`](Self::shader_values).
	pub fn constants(&self) -> &[Constant] {
		&self.constants
	}

	/// The pool the constants slice into.
	pub fn shader_values(&self) -> &[f32] {
		&self.shader_values
	}

	/// The value a constant refers to, or `None` if it points outside the pool.
	pub fn constant_values(&self, constant: &Constant) -> Option<&[f32]> {
		let start = usize::from(constant.value_offset) / 4;
		let len = usize::from(constant.value_size) / 4;
		self.shader_values.get(start..start + len)
	}

	/// Per-row colour data, when the material carries a table.
	pub fn color_table(&self) -> Option<&ColorTable> {
		self.color_table.as_ref()
	}

	/// Trailing container bytes. The low four are the colour table's flags.
	pub fn additional_data(&self) -> &[u8] {
		&self.additional_data
	}
}

// Construction logic.
impl Material {
	fn string_at(strings: &[u8], offset: u16) -> Result<String> {
		let mut cursor = Cursor::new(strings);
		cursor.set_position(offset.into());
		Ok(NullString::read(&mut cursor)?.to_string())
	}

	fn read_attribute_sets(
		strings: &[u8],
		sets: &[structs::AttributeSet],
	) -> Result<Vec<AttributeSet>> {
		sets.iter()
			.map(|set| {
				Ok(AttributeSet {
					name: Self::string_at(strings, set.name_offset)?,
					index: set.index,
				})
			})
			.collect()
	}
}

impl File for Material {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		let file = structs::Material::read(&mut stream)?;
		let strings = &file.string_data;
		// The first four bytes of the trailing header describe the colour table. Its shape is stated
		// here rather than inferred from the block's size, which cannot tell a legacy table with a
		// dye table from an extended one without.
		let table_flags = match file.additional_data.get(..4) {
			Some(bytes) => u32::from_le_bytes(bytes.try_into().expect("four bytes")),
			None => 0,
		};

		let textures = file
			.texture_offsets
			.iter()
			.map(|texture| {
				Ok(Texture {
					path: Self::string_at(strings, texture.offset)?,
					flags: texture.flags,
				})
			})
			.collect::<Result<Vec<_>>>()?;

		let samplers = file
			.samplers
			.iter()
			.map(|sampler| Sampler {
				id: sampler.id,
				flags: sampler.flags,
				texture_index: sampler.texture_index,
			})
			.collect();

		Ok(Material {
			version: file.version,
			shader: Self::string_at(strings, file.shader_package_name_offset)?,
			shader_flags: file.shader_flags,
			uv_sets: Self::read_attribute_sets(strings, &file.uv_sets)?,
			color_sets: Self::read_attribute_sets(strings, &file.color_sets)?,
			textures,
			samplers,
			shader_keys: file
				.shader_keys
				.iter()
				.map(|key| ShaderKey {
					category: key.category,
					value: key.value,
				})
				.collect(),
			constants: file
				.constants
				.iter()
				.map(|constant| Constant {
					id: constant.id,
					value_offset: constant.value_offset,
					value_size: constant.value_size,
				})
				.collect(),
			shader_values: file.shader_values,
			color_table: file
				.color_table
				.map(|values| ColorTable::new(values, table_flags)),
			additional_data: file.additional_data,
		})
	}
}

impl fmt::Debug for Material {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Material")
			.field("version", &self.version)
			.field("shader", &self.shader)
			.field("textures", &self.textures)
			.field("samplers", &self.samplers)
			.field(
				"color_table",
				&self.color_table.as_ref().map(ColorTable::kind),
			)
			.finish_non_exhaustive()
	}
}

/// A texture referenced by a material.
#[derive(Debug, Clone, CopyGetters)]
pub struct Texture {
	path: String,
	#[get_copy = "pub"]
	flags: u16,
}

impl Texture {
	/// Path to the texture. Not guaranteed to be absolute.
	pub fn path(&self) -> &str {
		&self.path
	}

	/// Whether the DX11 variant is used, which prefixes the file name with `--`.
	pub fn dx11(&self) -> bool {
		self.flags & 0x8000 != 0
	}
}

/// A named index the shader can address, used for UV and colour sets.
#[derive(Debug, Clone, CopyGetters)]
pub struct AttributeSet {
	name: String,
	#[get_copy = "pub"]
	index: u16,
}

impl AttributeSet {
	/// Name of the set.
	pub fn name(&self) -> &str {
		&self.name
	}
}

/// How a sampler axis addresses a coordinate outside `[0, 1]`. Bit values follow Vulkan's
/// `VkSamplerAddressMode` / D3D11's `D3D11_TEXTURE_ADDRESS_MODE - 1`; `ClampToBorder` is declared
/// for completeness but has not been observed on a real material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
	ClampToBorder,
}

/// Texture sampler for a material.
#[derive(Debug, Clone, Copy, CopyGetters)]
pub struct Sampler {
	/// Sampler ID, which identifies what the bound texture is used for.
	#[get_copy = "pub"]
	id: u32,
	/// Sampler state: two bits of address mode per axis at 0/2/4, an anisotropy nibble at 6, and a
	/// signed 10-bit LOD bias fixed at 1/32 of a mip level at 10. Bits above 19 are unused.
	///
	/// Measured against a RenderDoc capture of the game's own `VkSamplerCreateInfo`: `hair.shpk`'s
	/// alpha-tested normal map states mirrored addressing, -1.0 bias and 16x anisotropy, and this
	/// decode reproduces all three exactly. Swept over 111,517 live materials, only 11 distinct
	/// values occur, address axes always agree with each other, and the bias field only ever holds
	/// 0, -1.0 or -2.0.
	#[get_copy = "pub"]
	flags: u32,
	texture_index: u8,
}

impl Sampler {
	/// Marks a sampler the material declares but binds no texture to. Shaders that need no texture,
	/// such as `verticalfog.shpk`, carry one of these and no textures at all.
	const UNBOUND: u8 = 0xFF;

	/// Index into [`Material::textures`], or `None` when nothing is bound.
	pub fn texture_index(&self) -> Option<u8> {
		match self.texture_index {
			Self::UNBOUND => None,
			index => Some(index),
		}
	}

	fn address(&self, shift: u32) -> AddressMode {
		match (self.flags >> shift) & 0b11 {
			0 => AddressMode::Repeat,
			1 => AddressMode::MirroredRepeat,
			2 => AddressMode::ClampToEdge,
			_ => AddressMode::ClampToBorder,
		}
	}

	/// Address mode of the sampler's first axis. Every material sampler in the live corpus states
	/// the same mode on all three axes, so which bit pair is U, V or W in the game's own terms is
	/// not independently confirmed - only that they decode identically and in the same order.
	pub fn address_u(&self) -> AddressMode {
		self.address(0)
	}

	/// Address mode of the sampler's second axis.
	pub fn address_v(&self) -> AddressMode {
		self.address(2)
	}

	/// Address mode of the sampler's third axis, read by a volume or cube sampler.
	pub fn address_w(&self) -> AddressMode {
		self.address(4)
	}

	/// The sampler's mip LOD bias. A signed 10-bit field at bits 10-19, scaled by 1/32; the corpus
	/// only ever states 0, -1.0 or -2.0.
	pub fn lod_bias(&self) -> f32 {
		let field = (self.flags >> 10) & 0x3ff;
		let signed = if field >= 0x200 {
			field as i32 - 0x400
		} else {
			field as i32
		};
		signed as f32 / 32.0
	}

	/// Whether the sampler asks for anisotropic filtering. The corpus states 16x wherever this is
	/// set; two rarer nibble values (21 and 4 of 111,517 materials) are not yet decoded and read as
	/// `false` here.
	pub fn anisotropic(&self) -> bool {
		(self.flags >> 6) & 0xf == 0b1101
	}
}

/// Selects a variant within the shader package.
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct ShaderKey {
	category: u32,
	value: u32,
}

/// Names a span of [`Material::shader_values`].
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Constant {
	id: u32,
	value_offset: u16,
	value_size: u16,
}

/// One decoded row of a colour table. Field meanings follow Penumbra.GameData's `ColorTableRow`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRow {
	pub diffuse: [f32; 3],
	pub specular: [f32; 3],
	pub emissive: [f32; 3],
	pub sheen_rate: f32,
	pub sheen_tint: f32,
	pub sheen_aperture: f32,
	pub roughness: f32,
	pub metalness: f32,
	pub anisotropy: f32,
	pub sphere_mask: f32,
	pub shader_index: u16,
	pub tile_index: u16,
	pub tile_alpha: f32,
	pub sphere_index: u16,
	/// Row-major 2x2 UV transform.
	pub tile_transform: [f32; 4],
}

/// A field of a colour table row a dye can drive. Bit positions follow Penumbra.GameData's
/// `ColorDyeTableRow`, and the order is the one staining templates hold their columns in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DyeField {
	Diffuse,
	Specular,
	Emissive,
	Scalar3,
	Metalness,
	Roughness,
	SheenRate,
	SheenTint,
	SheenAperture,
	Anisotropy,
	SphereIndex,
	SphereMask,
}

/// One row of a dye table: the staining template the matching colour table row takes dyed values
/// from, and which of its fields it takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyeRow {
	template: u16,
	channel: u8,
	fields: u16,
}

impl DyeRow {
	/// Template the dyed values come from, which also says which of the two staining template files
	/// holds it. Zero where the row is not dyed at all.
	pub fn template(&self) -> u16 {
		self.template
	}

	/// Which of the item's stains the row reads, for the two an item can carry. Always zero on a
	/// legacy table, which predates the second.
	pub fn channel(&self) -> u8 {
		self.channel
	}

	/// Whether the template drives this field. The colour table's own value stands where it does not.
	pub fn dyes(&self, field: DyeField) -> bool {
		self.fields & (1 << field as u16) != 0
	}
}

/// IEEE 754 binary16 to binary32.
fn half_to_f32(bits: u16) -> f32 {
	let sign = u32::from(bits & 0x8000) << 16;
	let exponent = u32::from(bits >> 10) & 0x1F;
	let mantissa = u32::from(bits & 0x3FF);
	let bits = match exponent {
		0 if mantissa == 0 => sign,
		// Subnormal: normalise by hand, since the exponent has no direct binary32 equivalent.
		0 => {
			let shift = mantissa.leading_zeros() - 21;
			sign | ((127 - 15 - shift) << 23) | ((mantissa << (shift + 1)) & 0x7F_FFFF)
		}
		0x1F => sign | 0x7F80_0000 | (mantissa << 13),
		_ => sign | ((exponent + 127 - 15) << 23) | (mantissa << 13),
	};
	f32::from_bits(bits)
}

/// Which layout a material's colour table uses. They differ in row width and row count, and only the
/// total size tells them apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTableKind {
	/// 16 rows of 16 halves, used before Dawntrail.
	Legacy,
	/// 32 rows of 32 halves.
	Extended,
	/// Size matches neither layout; rows are not split out.
	Unknown,
}

/// Per-row colour data a material applies to the surfaces that reference it.
#[derive(Debug, Clone)]
pub struct ColorTable {
	values: Vec<u16>,
	dye: Vec<u16>,
	kind: ColorTableKind,
}

impl ColorTable {
	const LEGACY_ROW: usize = 16;
	const EXTENDED_ROW: usize = 32;

	fn new(mut values: Vec<u16>, table_flags: u32) -> Self {
		// Bits 4..12 hold the base-2 logs of the table's dimensions, which is what distinguishes the
		// layouts. Both zero is the pre-Dawntrail table.
		let kind = match ((table_flags >> 4) & 0xFF) as u8 {
			0x00 | 0x42 => ColorTableKind::Legacy,
			0x53 => ColorTableKind::Extended,
			_ => ColorTableKind::Unknown,
		};
		let table_len = match kind {
			ColorTableKind::Legacy => Self::LEGACY_ROW * 16,
			ColorTableKind::Extended => Self::EXTENDED_ROW * 32,
			ColorTableKind::Unknown => values.len(),
		};
		// Some materials declare a table without writing one, so a short block is truncated rather
		// than treated as a failure.
		let dye = values.split_off(table_len.min(values.len()));
		Self { values, dye, kind }
	}

	/// Whether the material declares a colour table at all.
	pub fn declared(table_flags: u32) -> bool {
		table_flags & 0x4 != 0
	}

	/// Whether a dye table follows the colour table.
	pub fn has_dye(table_flags: u32) -> bool {
		table_flags & 0x8 != 0
	}

	/// The dye table trailing the colour table, empty when the material has none.
	/// [`dye_row`](Self::dye_row) reads one row of it.
	pub fn dye(&self) -> &[u16] {
		&self.dye
	}

	/// One row of the dye table, or `None` where the material carries none for that row. A legacy
	/// row is a `u16` naming five fields; an extended one is a `u32` naming twelve.
	pub fn dye_row(&self, index: usize) -> Option<DyeRow> {
		match self.kind {
			ColorTableKind::Legacy => {
				let bits = *self.dye.get(index)?;
				Some(DyeRow {
					template: bits >> 5,
					channel: 0,
					fields: bits & 0x1F,
				})
			}
			ColorTableKind::Extended => {
				let halves = self.dye.get(index * 2..index * 2 + 2)?;
				let bits = u32::from(halves[0]) | (u32::from(halves[1]) << 16);
				Some(DyeRow {
					template: ((bits >> 16) & 0x7FF) as u16,
					channel: ((bits >> 27) & 0x3) as u8,
					fields: (bits & 0xFFF) as u16,
				})
			}
			ColorTableKind::Unknown => None,
		}
	}

	/// Which layout this table uses.
	pub fn kind(&self) -> ColorTableKind {
		self.kind
	}

	/// Number of rows, or zero when the layout is not recognised.
	pub fn rows(&self) -> usize {
		match self.row_width() {
			Some(width) => self.values.len() / width,
			None => 0,
		}
	}

	/// One row's raw halves.
	pub fn row(&self, index: usize) -> Option<&[u16]> {
		let width = self.row_width()?;
		self.values.get(index * width..(index + 1) * width)
	}

	/// The table as stored, for callers that want to interpret it themselves.
	pub fn raw(&self) -> &[u16] {
		&self.values
	}

	/// One row, decoded. Rows are stored as IEEE half floats; the extended layout carries every
	/// field, while the legacy one stops after the tile transform's first pair.
	pub fn row_values(&self, index: usize) -> Option<ColorRow> {
		let row = self.row(index)?;
		let at = |i: usize| row.get(i).copied().map(half_to_f32).unwrap_or(0.0);
		Some(ColorRow {
			diffuse: [at(0), at(1), at(2)],
			specular: [at(4), at(5), at(6)],
			emissive: [at(8), at(9), at(10)],
			sheen_rate: at(12),
			sheen_tint: at(13),
			sheen_aperture: at(14),
			roughness: at(16),
			metalness: at(18),
			anisotropy: at(19),
			sphere_mask: at(21),
			shader_index: at(24) as u16,
			tile_index: (at(25) * 64.) as u16,
			tile_alpha: at(26),
			sphere_index: at(27) as u16,
			tile_transform: [at(28), at(29), at(30), at(31)],
		})
	}

	fn row_width(&self) -> Option<usize> {
		match self.kind {
			ColorTableKind::Legacy => Some(Self::LEGACY_ROW),
			ColorTableKind::Extended => Some(Self::EXTENDED_ROW),
			ColorTableKind::Unknown => None,
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use crate::file::File;

	use super::{ColorTableKind, DyeField, Material};

	/// A material carrying nothing but a colour table of the stated dimensions and a dye table.
	fn material(logs: u32, table: &[u16]) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.extend(0x0103_0000u32.to_le_bytes());
		bytes.extend(0u16.to_le_bytes());
		bytes.extend(u16::try_from(table.len() * 2).unwrap().to_le_bytes());
		bytes.extend(1u16.to_le_bytes());
		bytes.extend(0u16.to_le_bytes());
		// No textures, UV sets or colour sets, and four bytes of trailing header.
		bytes.extend([0, 0, 0, 4]);
		// The shader name, which is only its terminator.
		bytes.push(0);
		bytes.extend((0xC | (logs << 4)).to_le_bytes());
		bytes.extend(table.iter().flat_map(|half| half.to_le_bytes()));
		// Material header, declaring no keys, constants, samplers or values.
		bytes.extend([0; 12]);
		bytes
	}

	#[test]
	fn splits_an_extended_dye_row() {
		let row = |template: u32, channel: u32, fields: u32| {
			let bits = fields | (template << 16) | (channel << 27);
			[bits as u16, (bits >> 16) as u16]
		};
		let mut table = vec![0u16; 32 * 32];
		table.extend([row(0x2AB, 1, 0x801), row(0, 0, 0)].concat());

		let file = Material::read(Cursor::new(material(0x53, &table))).unwrap();
		let table = file.color_table().unwrap();
		assert_eq!(table.kind(), ColorTableKind::Extended);

		let dye = table.dye_row(0).unwrap();
		assert_eq!(dye.template(), 0x2AB);
		assert_eq!(dye.channel(), 1);
		assert!(dye.dyes(DyeField::Diffuse));
		assert!(dye.dyes(DyeField::SphereMask));
		assert!(!dye.dyes(DyeField::Specular));
		assert_eq!(table.dye_row(1).unwrap().template(), 0);
		assert!(table.dye_row(2).is_none());
	}

	/// A legacy row is half the width, names five fields, and has no channel.
	#[test]
	fn splits_a_legacy_dye_row() {
		let mut table = vec![0u16; 16 * 16];
		table.push((200 << 5) | 0x11);

		let file = Material::read(Cursor::new(material(0x00, &table))).unwrap();
		let table = file.color_table().unwrap();
		assert_eq!(table.kind(), ColorTableKind::Legacy);

		let dye = table.dye_row(0).unwrap();
		assert_eq!(dye.template(), 200);
		assert_eq!(dye.channel(), 0);
		assert!(dye.dyes(DyeField::Diffuse));
		assert!(dye.dyes(DyeField::Metalness));
		assert!(!dye.dyes(DyeField::Roughness));
		assert!(table.dye_row(1).is_none());
	}

	/// A tile index is stored as `(index + 0.5) / 64`.
	#[test]
	fn scales_a_tile_index() {
		let mut table = vec![0u16; 32 * 32];
		table[25] = 0x2D80;
		table[32 + 25] = 0x3BF0;

		let file = Material::read(Cursor::new(material(0x53, &table))).unwrap();
		let table = file.color_table().unwrap();
		assert_eq!(table.row_values(0).unwrap().tile_index, 5);
		assert_eq!(table.row_values(1).unwrap().tile_index, 63);
	}

	#[test]
	fn reads_no_row_from_a_table_it_cannot_shape() {
		let file = Material::read(Cursor::new(material(0x02, &[0; 8]))).unwrap();
		let table = file.color_table().unwrap();
		assert_eq!(table.kind(), ColorTableKind::Unknown);
		assert!(table.dye_row(0).is_none());
	}

	/// `hair.shpk`'s slot-0 sampler, measured off a RenderDoc capture: mirrored on every axis,
	/// -1.0 LOD bias and 16x anisotropy.
	#[test]
	fn decodes_the_hair_normal_map_sampler() {
		let sampler = super::Sampler {
			id: 0,
			flags: 0x000f_8355,
			texture_index: 0,
		};
		assert_eq!(sampler.address_u(), super::AddressMode::MirroredRepeat);
		assert_eq!(sampler.address_v(), super::AddressMode::MirroredRepeat);
		assert_eq!(sampler.address_w(), super::AddressMode::MirroredRepeat);
		assert_eq!(sampler.lod_bias(), -1.0);
		assert!(sampler.anisotropic());
	}

	/// `bg.shpk` terrain color maps state the same address and anisotropy but no bias.
	#[test]
	fn decodes_a_terrain_sampler_with_no_bias() {
		let sampler = super::Sampler {
			id: 0,
			flags: 0x0000_0340,
			texture_index: 0,
		};
		assert_eq!(sampler.address_u(), super::AddressMode::Repeat);
		assert_eq!(sampler.lod_bias(), 0.0);
		assert!(sampler.anisotropic());
	}

	/// The rare skin variant states twice hair's bias.
	#[test]
	fn decodes_a_double_bias_sampler() {
		let sampler = super::Sampler {
			id: 0,
			flags: 0x000f_0355,
			texture_index: 0,
		};
		assert_eq!(sampler.lod_bias(), -2.0);
		assert!(sampler.anisotropic());
	}

	/// An unbound sampler's all-zero flags decode to the identity state.
	#[test]
	fn decodes_an_unbound_sampler_as_neutral() {
		let sampler = super::Sampler {
			id: 0,
			flags: 0,
			texture_index: super::Sampler::UNBOUND,
		};
		assert_eq!(sampler.address_u(), super::AddressMode::Repeat);
		assert_eq!(sampler.lod_bias(), 0.0);
		assert!(!sampler.anisotropic());
	}
}
