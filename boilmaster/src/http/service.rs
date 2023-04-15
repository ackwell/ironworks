use std::sync::Arc;

use axum::extract::FromRef;

use crate::{data, schema, search2, version};

pub type Data = Arc<data::Data>;
pub type Schema = Arc<schema::Provider>;
pub type Search = Arc<search2::Search>;
pub type Version = Arc<version::Manager>;

#[derive(Clone, FromRef)]
pub struct State {
	pub data: Data,
	pub schema: Schema,
	pub search: Search,
	pub version: Version,
}
