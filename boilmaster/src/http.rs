use std::io::Read;

use axum::{response::IntoResponse, routing::get, Router};
use ironworks::{ffxiv, sqpack::SqPack};

pub fn router() -> Router {
	Router::new().route("/sheets", get(sheets))
}

async fn sheets() -> impl IntoResponse {
	// TODO this should not be done every request
	// TODO probably need something in iw::excel for listing sheet names publicly
	let sqpack = SqPack::new(ffxiv::FsResource::search().unwrap());
	let mut root = sqpack.file("exd/root.exl").unwrap();
	let mut string = String::new();
	root.read_to_string(&mut string).unwrap();

	string
}
