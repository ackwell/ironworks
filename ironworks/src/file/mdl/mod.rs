//! Structs and utilities for parsing .mdl files.

// TODO: REMOVE
#![allow(missing_docs)]

mod container;
mod mesh;
mod model;
mod structs;

pub use {
	container::ModelContainer,
	mesh::{Mesh, VertexAttribute, VertexValues},
	model::{Lod, Model},
	structs::VertexAttributeKind,
};
