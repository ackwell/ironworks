use std::sync::Arc;

use axum::extract::FromRef;

use crate::{asset, data, schema, search, version};

pub type Asset = Arc<asset::Service>;
pub type Data = Arc<data::Data>;
pub type Schema = Arc<schema::Provider>;
pub type Search = Arc<search::Search>;
pub type Version = Arc<version::Manager>;

#[derive(Clone, FromRef)]
pub struct State {
	pub asset: Asset,
	pub data: Data,
	pub schema: Schema,
	pub search: Search,
	pub version: Version,
}
