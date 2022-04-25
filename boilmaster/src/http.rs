use std::{fs, io::Read};

use axum::{response::IntoResponse, routing::get, Extension, Router};
use ironworks::{
	ffxiv,
	sqpack::{File, SqPack},
};
use tokio::sync::{
	mpsc::{self, Sender},
	oneshot,
};

// todo this shouldn't be in http
#[derive(Debug)]
enum IronworksRequest {
	SheetList {
		responder: oneshot::Sender<File<fs::File>>,
	},
}

pub fn router() -> Router {
	// IW isn't async, nor send/sync. Boot up a channel so we can serve requests from a single location.
	// TODO: this seems sane to me but idk maybe iw should be async? idk.
	let (tx, mut rx) = mpsc::channel::<IronworksRequest>(32);

	tokio::spawn(async move {
		let sqpack = SqPack::new(ffxiv::FsResource::search().unwrap());

		while let Some(request) = rx.recv().await {
			use IronworksRequest::*;
			match request {
				SheetList { responder } => {
					// TODO probably need something in iw::excel for listing sheet names publicly
					let file = sqpack.file("exd/root.exl").unwrap();
					responder.send(file).ok();
				}
			}
		}
	});

	Router::new()
		.route("/sheets", get(sheets))
		.layer(Extension(tx))
}

async fn sheets(Extension(tx): Extension<Sender<IronworksRequest>>) -> impl IntoResponse {
	let (res_tx, res_rx) = oneshot::channel();
	tx.send(IronworksRequest::SheetList { responder: res_tx })
		.await
		.unwrap();

	let mut response = res_rx.await.unwrap();

	// TODO this should not be done every request
	let mut string = String::new();
	response.read_to_string(&mut string).unwrap();

	string
}
