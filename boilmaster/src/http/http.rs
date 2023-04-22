use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{Router, Server};
use futures::Future;
use serde::Deserialize;
use tower_http::trace::TraceLayer;

use super::{admin, asset, search, service, sheets};

#[derive(Debug, Deserialize)]
pub struct Config {
	admin: admin::Config,

	address: Option<IpAddr>,
	port: u16,
}

pub async fn serve(
	shutdown: impl Future<Output = ()>,
	config: Config,
	data: service::Data,
	asset: service::Asset,
	schema: service::Schema,
	search: service::Search,
	version: service::Version,
) -> Result<()> {
	let bind_address = SocketAddr::new(
		config.address.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
		config.port,
	);

	tracing::info!("http binding to {bind_address:?}");

	let router = Router::new()
		.nest("/admin", admin::router(config.admin))
		.nest("/asset", asset::router())
		.nest("/sheets", sheets::router())
		.nest("/search", search::router())
		.layer(TraceLayer::new_for_http())
		.with_state(service::State {
			asset,
			data,
			schema,
			search,
			version,
		});

	Server::bind(&bind_address)
		.serve(router.into_make_service())
		.with_graceful_shutdown(shutdown)
		.await
		.unwrap();

	Ok(())
}
