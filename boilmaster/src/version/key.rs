use std::{fmt, num::ParseIntError, str::FromStr};

use serde::{de, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionKey(u32);

impl VersionKey {
	pub fn from_latest_patches(latest_patches: &[impl AsRef<str>]) -> Self {
		let bytes = latest_patches
			.iter()
			.flat_map(|v| v.as_ref().as_bytes())
			.copied()
			.collect::<Vec<_>>();
		let hash = murmurhash32::murmurhash3(&bytes);

		Self(hash)
	}
}

impl fmt::Display for VersionKey {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_fmt(format_args!("{:x}", self.0))
	}
}

impl FromStr for VersionKey {
	type Err = ParseIntError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		u32::from_str_radix(input, 16).map(VersionKey)
	}
}

impl Serialize for VersionKey {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for VersionKey {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;
		raw.parse().map_err(de::Error::custom)
	}
}
