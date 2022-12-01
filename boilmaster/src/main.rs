use std::sync::Arc;

use boilmaster::{data::Data, http, search::Search};
use figment::{
	providers::{Format, Toml},
	Figment,
};
use serde::Deserialize;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
pub struct Config {
	http: http::Config,
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
	// TODO: env filter (will need feature enabled). consider enabling pulling from log! too. do i try and read config from a file manually ala asp config or go all in with a dotenv?
	let filter = filter::Targets::new()
		.with_default(Level::DEBUG)
		.with_target("tantivy", Level::WARN);

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
