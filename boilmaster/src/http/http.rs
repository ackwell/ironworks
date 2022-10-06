use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router, Server};
use tokio::signal;
use tower_http::trace::TraceLayer;

use crate::{data::Data, search::Search};

use super::{search, sheets};

// TODO: should the data be an arc at this point? i guess it'll depend if i need to store refs to it for search &c
pub async fn serve(data: Data, search: Search) {
	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(router(data, search).into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap()
}

fn router(data: Data, search: Search) -> Router {
	Router::new()
		.nest("/sheets", sheets::router())
		.nest("/search", search::router(search))
		// TODO: I'm not convinced by setting up the data layer this high, seems a bit magic so to speak
		.layer(Extension(Arc::new(data)))
		.layer(TraceLayer::new_for_http())
}

// TODO: can I set this up in the main.rs and use it in multiple places?
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
