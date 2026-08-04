use std::{
	io::{Read, Seek},
	sync::Arc,
};

use binrw::BinRead;

use crate::{error::Result, file::FormatRead};

use super::{
	model::{Lod, Model},
	structs,
};

/// A model container file, holding one or more models and related metadata.
#[derive(Debug)]
pub struct ModelContainer {
	file: Arc<structs::File>,
}

impl FormatRead for ModelContainer {
	fn from_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
		let file = structs::File::read(&mut reader)?;
		Ok(ModelContainer { file: file.into() })
	}
}

impl ModelContainer {
	// TODO: consider how variants will work
	// TODO: some stuff doesn't have models at lower lods - should that be exposed at this level?
	/// Get the model for the specified LOD.
	pub fn model(&self, level: Lod) -> Model {
		Model {
			file: self.file.clone(),

			level,
		}
	}
}
