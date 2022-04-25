use std::net::SocketAddr;

use axum::Server;
use boilmaster::http;
use tokio::signal;

#[tokio::main]
async fn main() {
	let app = http::router();

	// TODO: not sure if this should be here or http. think about it.
	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(app.into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap()
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
