use std::{collections::HashMap, fmt, str::FromStr, sync::Arc};

use boilmaster::{data::Data, http, search::Search};
use figment::{
	providers::{Format, Toml},
	Figment,
};
use serde::{de, Deserialize};
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
struct Config {
	tracing: Tracing,
	http: http::Config,
}

// TODO: tracing should proooobably be it's own file at this point
#[derive(Debug, Deserialize)]
struct Tracing {
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

#[tokio::main]
async fn main() {
	// Load configuration
	// TODO: is it worth having a cli flag to specify the config path or is that just immense overkill?
	let config = Figment::new()
		.merge(Toml::file("boilmaster.toml"))
		.extract::<Config>()
		.expect("TODO: Error handling");

	// Set up tracing
	let filter = filter::Targets::new()
		.with_default(config.tracing.filters.default)
		.with_targets(config.tracing.filters.targets);

	// TODO: env filter (will need feature enabled). consider enabling pulling from log! too.
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.with(filter)
		.init();

	let data = Arc::new(Data::new());
	let search = Arc::new(Search::new());

	// Set up a cancellation token that will fire when a shutdown signal is recieved.
	let shutdown_token = shutdown_token();

	let (ingest_result, _) = tokio::join!(
		search
			.clone()
			.ingest(shutdown_token.cancelled(), &data, None),
		http::serve(
			shutdown_token.cancelled(),
			config.http,
			data.clone(),
			search
		),
	);

	// TODO: when ingesting multiple versions, should probably bundle the ingests up side by side, but handle errors properly between them
	ingest_result.expect("TODO: Error handling")
}

fn shutdown_token() -> CancellationToken {
	// Create a token to represent the shutdown signal.
	let token = CancellationToken::new();

	// Set up a background task to wait for the signal with a copy of the token.
	let inner_token = token.clone();
	tokio::spawn(async move {
		shutdown_signal().await;
		inner_token.cancel();
	});

	// Return the pending token for use.
	token
}

async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("Failed to install Ctrl+C handler.");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("Failed to install SIGTERM handler.")
			.recv()
			.await
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	println!("Shutdown signal received.")
}
