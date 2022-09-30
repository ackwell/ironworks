use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router, Server};
use ironworks::{excel::Excel, ffxiv, sqpack::SqPack, Ironworks};
use tokio::signal;
use tower_http::trace::TraceLayer;

use crate::search::temp_test_search;

use super::sheets;

pub async fn serve() {
	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(router().into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap()
}

fn router() -> Router {
	// TODO: Work out how to handle languages
	let ironworks =
		Ironworks::new().with_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
	let excel = Excel::with()
		.language(ffxiv::Language::English)
		.build(Arc::new(ironworks), ffxiv::Mapper::new());

	// TODO: THIS SHOULD NOT BE HERE
	temp_test_search(&excel).unwrap();

	Router::new()
		.nest("/sheets", sheets::router())
		.layer(Extension(Arc::new(excel)))
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
