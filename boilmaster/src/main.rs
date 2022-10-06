use boilmaster::{data::Data, http, search::Search};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[tokio::main]
async fn main() {
	// Set up tracing
	// TODO: env filter (will need feature enabled). consider enabling pulling from log! too. do i try and read config from a file manually ala asp config or go all in with a dotenv?
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().with_filter(filter::LevelFilter::DEBUG))
		.init();

	let data = Data::new();

	let mut search = Search::new();
	search.initialize(&data).expect("TODO: Error handling here");

	http::serve(data).await
}
