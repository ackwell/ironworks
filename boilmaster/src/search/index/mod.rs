mod index;
mod ingest;
mod resolve;
mod schema;

pub use {
	index::{Index, IndexResult},
	ingest::{IngestConfig, Ingester},
};
