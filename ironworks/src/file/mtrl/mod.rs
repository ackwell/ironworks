//! Structs and utilities for parsing .mtrl files.

mod material;
mod structs;

pub use material::{
	AddressMode, AttributeSet, ColorRow, ColorTable, ColorTableKind, Constant, DyeField, DyeRow,
	Material, Sampler, ShaderKey, Texture,
};
