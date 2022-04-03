//! Tools for working with the SqPack package format.

mod ffxiv;
mod file;
mod index;
mod resource;
mod sqpack;

pub use {ffxiv::FfxivFsResource, file::File, resource::Resource, sqpack::SqPack};
