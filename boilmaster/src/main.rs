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

	let data = Data::new();

	let mut search = Search::new();
	search.ingest(&data, None).expect("TODO: Error handling");

	http::serve(data, search).await
}
