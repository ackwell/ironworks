use std::{fmt, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VersionKey(String);

impl fmt::Display for VersionKey {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(formatter)
	}
}

impl AsRef<Path> for VersionKey {
	fn as_ref(&self) -> &Path {
		self.0.as_ref()
	}
}

impl VersionKey {
	pub fn from_latest_patches(latest_patches: &[impl AsRef<str>]) -> Self {
		let bytes = latest_patches
			.iter()
			.flat_map(|v| v.as_ref().as_bytes())
			.copied()
			.collect::<Vec<_>>();
		let hash = murmurhash32::murmurhash3(&bytes);

		Self(format!("{hash:x}"))
	}
}
