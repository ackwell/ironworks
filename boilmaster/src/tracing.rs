use std::{collections::HashMap, fmt, str::FromStr};

use serde::{de, Deserialize};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

// TODO: tracing should proooobably be it's own file at this point
#[derive(Debug, Deserialize)]
pub struct Config {
	// TODO: log file config? or like, sink config? work out how that's going to work i guess.
	filters: TracingFilters,
}

#[derive(Debug, Deserialize)]
struct TracingFilters {
	default: ConfigLevelFilter,

	#[serde(flatten)]
	targets: HashMap<String, ConfigLevelFilter>,
}

#[repr(transparent)]
struct ConfigLevelFilter(LevelFilter);

impl From<ConfigLevelFilter> for LevelFilter {
	fn from(filter: ConfigLevelFilter) -> Self {
		filter.0
	}
}

impl fmt::Debug for ConfigLevelFilter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl<'de> Deserialize<'de> for ConfigLevelFilter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		let level_filter = LevelFilter::from_str(&string).map_err(de::Error::custom)?;
		Ok(Self(level_filter))
	}
}

pub fn init(config: Config) {
	let filter = filter::Targets::new()
		.with_default(config.filters.default)
		.with_targets(config.filters.targets);

	// TODO: env filter (will need feature enabled). consider enabling pulling from log! too.
	// TODO: now that i have config working, is it worth using env filter here or should i handle it via config env?
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.with(filter)
		.init();
}
