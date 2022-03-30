//! Tools for working with the SqPack package format.

mod ffxiv;
mod file;
mod index;
mod resource;
mod sqpack;

pub use ffxiv::FfxivFsResource;
pub use resource::Resource;
pub use sqpack::SqPack;
