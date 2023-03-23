use askama::Template;
use axum::{debug_handler, response::IntoResponse, routing::get, Router};

use super::{error::Result, service};

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate;

pub fn router() -> Router<service::State> {
	Router::new().route("/", get(admin))
}

#[debug_handler]
async fn admin() -> Result<impl IntoResponse> {
	let template = AdminTemplate;
	Ok(template)
}
