use std::str::FromStr;

use serde::{de, Deserialize};

use super::{convert, error::Error};

#[derive(Debug, Clone, Copy)]
pub enum Format {
	Png,
}

impl Format {
	pub fn extension(&self) -> &str {
		match self {
			Self::Png => "png",
		}
	}

	pub(super) fn converter(&self) -> &dyn convert::Converter {
		match self {
			Self::Png => &convert::Image,
		}
	}
}

impl FromStr for Format {
	type Err = Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		Ok(match input {
			"png" => Self::Png,
			other => return Err(Error::UnknownFormat(other.into())),
		})
	}
}

impl<'de> Deserialize<'de> for Format {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;
		raw.parse().map_err(de::Error::custom)
	}
}
