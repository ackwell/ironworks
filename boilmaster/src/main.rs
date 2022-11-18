use std::sync::Arc;

use boilmaster::{data::Data, http, search::Search};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
	// Set up tracing
	// TODO: env filter (will need feature enabled). consider enabling pulling from log! too. do i try and read config from a file manually ala asp config or go all in with a dotenv?
	let filter = filter::Targets::new()
		.with_default(Level::DEBUG)
		.with_target("tantivy", Level::WARN);

	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.with(filter)
		.init();

	let data = Arc::new(Data::new());
	let search = Arc::new(Search::new());

	// TODO: At the moment, this results in the shutdown signal killing the server, but not effecting the search ingestion
	let (ingest_result, _) = tokio::join!(
		search.clone().ingest(&data, None),
		http::serve(data.clone(), search),
	);

	// TODO: when ingesting multiple versions, should probably bundle the ingests up side by side, but handle errors properly between them
	ingest_result.expect("TODO: Error handling")
}
