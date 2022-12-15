mod index;
mod ingest;
mod resolve;

pub use {
	index::{Index, IndexResult},
	ingest::{IngestConfig, Ingester},
};
