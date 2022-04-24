use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router, Server};

#[tokio::main]
async fn main() {
	let app = Router::new().route("/", get(test));

	Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
		.serve(app.into_make_service())
		.await
		.unwrap()
}

async fn test() -> impl IntoResponse {
	"Hello world."
}
