#![allow(clippy::module_inception)]

// TODO: probably take these non-public and expose an explicit interface here? or is it not worth it given this is the entry point
pub mod data;
pub mod http;
pub mod patch;
mod read;
pub mod schema;
pub mod search;
pub mod tracing;
mod utility;
