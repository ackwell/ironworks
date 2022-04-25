use std::net::SocketAddr;

use axum::Server;
use boilmaster::http;

#[tokio::main]
async fn main() {
	let app = http::router();

	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(app.into_make_service())
		.await
		.unwrap()
}
