//! Structs and utilities for parsing .mdl files.

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
