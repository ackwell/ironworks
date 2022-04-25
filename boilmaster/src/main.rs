use std::net::SocketAddr;

use axum::Server;
use boilmaster::http;

#[tokio::main]
async fn main() {
	let app = http::router();

	// TODO: not sure if this should be here or http. think about it.
	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(app.into_make_service())
		.await
		.unwrap()
}
