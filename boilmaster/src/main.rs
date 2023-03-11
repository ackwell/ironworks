use std::sync::Arc;

use boilmaster::{data::Data, http, patch, schema, search, tracing};
use figment::{
	providers::{Env, Format, Toml},
	Figment,
};
use serde::Deserialize;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Deserialize)]
struct Config {
	tracing: tracing::Config,
	http: http::Config,
	patch: patch::Config,
	schema: schema::Config,
	search: search::Config,
}

#[tokio::main]
async fn main() {
	// Load configuration
	// TODO: is it worth having a cli flag to specify the config path or is that just immense overkill?
	let config = Figment::new()
		.merge(Toml::file("boilmaster.toml"))
		.merge(Env::prefixed("BM_").split("_"))
		.extract::<Config>()
		.expect("TODO: Error handling");

	// Initialise tracing before getting too far into bootstrapping the rest of the application
	tracing::init(config.tracing);

	let view = patch::wip_build_zipatch_view(config.patch)
		.await
		.expect("TODO");

	let data = Arc::new(Data::new(view));
	let schema = Arc::new(schema::Provider::new(config.schema).expect("TODO: Error handling"));
	let search = Arc::new(search::Search::new(config.search));

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
			schema,
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
