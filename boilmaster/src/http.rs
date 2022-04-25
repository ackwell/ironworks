use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
	Router::new().route("/", get(hello_world))
}

async fn hello_world() -> impl IntoResponse {
	"Hello world."
}
