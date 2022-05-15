use std::{io::Cursor, sync::Arc};

use binrw::BinRead;

use crate::{error::Result, file::File};

use super::{
	model::{Lod, Model},
	structs,
};

#[derive(Debug)]
pub struct ModelContainer {
	file: Arc<structs::File>,
}

impl File for ModelContainer {
	fn read(data: Vec<u8>) -> Result<Self> {
		let file = structs::File::read(&mut Cursor::new(data))?;

		Ok(ModelContainer { file: file.into() })
	}
}

impl ModelContainer {
	// TODO: consider how variants will work
	pub fn model(&self, level: Lod) -> Model {
		Model {
			file: self.file.clone(),

			level,
		}
	}
}
