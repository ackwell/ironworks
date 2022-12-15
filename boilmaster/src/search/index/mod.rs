mod index;
mod ingest;

pub use {
	index::{Index, IndexResult},
	ingest::{IngestConfig, Ingester},
};
