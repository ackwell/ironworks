use std::{fmt, str::FromStr};

use anyhow::bail;
use ironworks::excel::Language;
use serde::de;

#[derive(Clone, Copy)]
pub struct LanguageString(Language);

impl fmt::Debug for LanguageString {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(formatter)
	}
}

impl From<LanguageString> for Language {
	fn from(wrapper: LanguageString) -> Self {
		wrapper.0
	}
}

impl FromStr for LanguageString {
	type Err = anyhow::Error;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		let language = match string {
			"ja" => Language::Japanese,
			"en" => Language::English,
			"de" => Language::German,
			"fr" => Language::French,
			"chs" => Language::ChineseSimplified,
			"cht" => Language::ChineseTraditional,
			"kr" => Language::Korean,
			_ => bail!("unrecognised language \"{string}\""),
		};

		Ok(Self(language))
	}
}

impl<'de> de::Deserialize<'de> for LanguageString {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;
		raw.parse().map_err(de::Error::custom)
	}
}
