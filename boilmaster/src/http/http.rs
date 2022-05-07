use std::sync::Arc;

use axum::{Extension, Router};
use ironworks::{excel::Excel, ffxiv, sqpack::SqPack, Ironworks};
use tower_http::trace::TraceLayer;

use super::sheets;

pub fn router() -> Router {
	// TODO: Work out how to handle languages
	let ironworks =
		Ironworks::new().with_resource(SqPack::new(ffxiv::FsResource::search().unwrap()));
	let excel = Excel::with()
		.language(ffxiv::Language::English)
		.build(Arc::new(ironworks), ffxiv::Mapper::new());

	Router::new()
		.nest("/sheets", sheets::router())
		.layer(Extension(Arc::new(excel)))
		.layer(TraceLayer::new_for_http())
}
