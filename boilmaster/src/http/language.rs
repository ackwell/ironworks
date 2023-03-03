use std::str::FromStr;

use anyhow::bail;
use ironworks::excel::Language;
use serde::de;

pub struct LanguageWrapper(Language);

impl From<LanguageWrapper> for Language {
	fn from(wrapper: LanguageWrapper) -> Self {
		wrapper.0
	}
}

impl FromStr for LanguageWrapper {
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

impl<'de> de::Deserialize<'de> for LanguageWrapper {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;
		raw.parse().map_err(de::Error::custom)
	}
}
