mod material;
mod pipeline;
mod plugin;

pub use {
	material::{Material, MaterialKind, MeshBundle},
	pipeline::{ATTRIBUTE_COLOR, ATTRIBUTE_UV_4},
	plugin::RenderPlugin,
};
