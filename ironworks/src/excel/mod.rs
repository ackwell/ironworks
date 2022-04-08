//! Tools for working with the Excel database format.

mod excel;
mod excel_options;
#[cfg(feature = "sqpack")]
mod ffxiv;
mod list;
mod resource;
mod sheet;

pub use {
	excel::Excel,
	resource::Resource,
	sheet::{Row, RowOptions, Sheet},
};

#[cfg(feature = "sqpack")]
pub use ffxiv::{FfxivSqpackResource, Language};
