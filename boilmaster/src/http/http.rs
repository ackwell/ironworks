use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{Extension, Router, Server};
use futures::Future;
use tower_http::trace::TraceLayer;

use crate::{data::Data, search::Search};

use super::{search, sheets};

pub async fn serve(
	shutdown: impl Future<Output = ()>,
	data: Arc<Data>,
	search: Arc<Search>,
) -> Result<()> {
	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(router(data, search).into_make_service())
		.with_graceful_shutdown(shutdown)
		.await
		.unwrap();

	Ok(())
}

fn router(data: Arc<Data>, search: Arc<Search>) -> Router {
	Router::new()
		.nest("/sheets", sheets::router())
		.nest("/search", search::router(search))
		// TODO: I'm not convinced by setting up the data layer this high, seems a bit magic so to speak
		.layer(Extension(data))
		.layer(TraceLayer::new_for_http())
}
