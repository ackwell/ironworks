use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router, Server};
use tokio::signal;
use tower_http::trace::TraceLayer;

use crate::{data::Data, search::temp_test_search};

use super::sheets;

pub async fn serve() {
	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(router().into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap()
}

fn router() -> Router {
	let data = Data::new();

	// TODO: THIS SHOULD NOT BE HERE
	temp_test_search(data.version(None).excel()).unwrap();

	Router::new()
		.nest("/sheets", sheets::router())
		.layer(Extension(Arc::new(data)))
		.layer(TraceLayer::new_for_http())
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
