use std::sync::Arc;

use boilmaster::{data, http, schema, search, search2, tracing, version};
use figment::{
	providers::{Env, Format, Toml},
	Figment,
};
use futures::TryFutureExt;
use serde::Deserialize;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Deserialize)]
struct Config {
	tracing: tracing::Config,
	http: http::Config,
	data: data::Config,
	version: version::Config,
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

	let version = Arc::new(version::Manager::new(config.version).expect("TODO"));
	let data = Arc::new(data::Data::new(config.data));
	let schema = Arc::new(schema::Provider::new(config.schema).expect("TODO: Error handling"));
	let search = Arc::new(search::Search::new(config.search));

	let search2 = search2::Search::new();

	// Set up a cancellation token that will fire when a shutdown signal is recieved.
	let shutdown_token = shutdown_token();

	tokio::try_join!(
		version.start(shutdown_token.child_token()),
		data.start(shutdown_token.child_token(), &version),
		search
			.start(shutdown_token.child_token(), &data)
			.map_err(anyhow::Error::from),
		//temp
		search2.start(shutdown_token.child_token(), &data),
		http::serve(
			shutdown_token.cancelled(),
			config.http,
			data.clone(),
			schema,
			search.clone(),
			version.clone(),
		),
	)
	.expect("TODO: Error handling");
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

	::tracing::info!("shutdown signal received");
}
