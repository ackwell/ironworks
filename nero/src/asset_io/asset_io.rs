use std::{
	io,
	path::{Path, PathBuf},
};

use bevy::asset::{AssetIo, AssetIoError, BoxedFuture};
use ironworks::ErrorValue;

use super::plugin::IronworksResource;

pub struct IronworksAssetIo {
	pub default_io: Box<dyn AssetIo>,

	pub ironworks: IronworksResource,
}

impl IronworksAssetIo {
	fn get_ironworks_path<'a>(&self, path: &'a Path) -> Option<&'a Path> {
		path.strip_prefix("iw://").ok()
	}
}

impl AssetIo for IronworksAssetIo {
	fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
		match self.get_ironworks_path(path) {
			Some(ironworks_path) => Box::pin(async move {
				self.ironworks
					.read()
					.unwrap()
					.file::<Vec<u8>>(&ironworks_path.to_string_lossy())
					.map_err(|error| match error {
						ironworks::Error::NotFound(ErrorValue::Path(path)) => {
							AssetIoError::NotFound(path.into())
						}
						other => AssetIoError::Io(io::Error::new(io::ErrorKind::Other, other)),
					})
			}),
			None => self.default_io.load_path(path),
		}
	}

	// The below just pass through to the base asset io, is it worth handling dirs, or changes for penumbra style resources?

	fn read_directory(
		&self,
		path: &std::path::Path,
	) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
		match self.get_ironworks_path(path) {
			Some(_) => Ok(Box::new(std::iter::empty())),
			None => self.default_io.read_directory(path),
		}
	}

	fn is_directory(&self, path: &Path) -> bool {
		match self.get_ironworks_path(path) {
			Some(_) => false,
			None => self.default_io.is_directory(path),
		}
	}

	fn watch_path_for_changes(&self, path: &Path) -> Result<(), AssetIoError> {
		match self.get_ironworks_path(path) {
			Some(_) => Ok(()),
			None => self.default_io.watch_path_for_changes(path),
		}
	}

	fn watch_for_changes(&self) -> Result<(), AssetIoError> {
		self.default_io.watch_for_changes()
	}
}
