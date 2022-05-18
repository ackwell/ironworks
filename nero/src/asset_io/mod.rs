mod asset_io;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod plugin;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use plugin::{IronworksAssetIoPlugin, IronworksState};
