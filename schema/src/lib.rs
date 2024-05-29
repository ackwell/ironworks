//! Schema types and parsers for defining the shape and semantics of FFXIV Excel
//! data structures.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]

mod error;
mod schema;

#[cfg(feature = "git2")]
mod git;

#[cfg(feature = "exdschema")]
pub mod exdschema;

#[cfg(feature = "saint_coinach")]
pub mod saint_coinach;

pub use {
	error::{Error, ErrorValue},
	schema::{
		Node, Order, ReferenceCondition, ReferenceTarget, Scalar, Schema, Sheet, StructField,
	},
};
