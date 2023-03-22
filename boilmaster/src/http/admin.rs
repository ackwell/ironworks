use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use axum_macros::debug_handler;

use super::error::Result;

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate;

pub fn router() -> Router {
	Router::new().route("/", get(admin))
}

#[debug_handler]
async fn admin() -> Result<impl IntoResponse> {
	let template = AdminTemplate;
	Ok(template.render().unwrap())
}
