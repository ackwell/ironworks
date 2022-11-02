use std::io;

use crate::{error::Result, sqpack};

// TODO: These (and path_metadata itself) should be moved into sqpack proper once and for all
const REPOSITORIES: &[&str] = &[
	"ffxiv", "ex1", "ex2", "ex3", "ex4", "ex5", "ex6", "ex7", "ex8", "ex9",
];

const CATEGORIES: &[Option<&str>] = &[
	/* 0x00 */ Some("common"),
	/* 0x01 */ Some("bgcommon"),
	/* 0x02 */ Some("bg"),
	/* 0x03 */ Some("cut"),
	/* 0x04 */ Some("chara"),
	/* 0x05 */ Some("shader"),
	/* 0x06 */ Some("ui"),
	/* 0x07 */ Some("sound"),
	/* 0x08 */ Some("vfx"),
	/* 0x09 */ Some("ui_script"),
	/* 0x0a */ Some("exd"),
	/* 0x0b */ Some("game_script"),
	/* 0x0c */ Some("music"),
	/* 0x0d */ None,
	/* 0x0e */ None,
	/* 0x0f */ None,
	/* 0x10 */ None,
	/* 0x11 */ None,
	/* 0x12 */ Some("sqpack_test"),
	/* 0x13 */ Some("debug"),
];

#[derive(Debug)]
pub struct Version {}

impl Version {
	pub(super) fn new() -> Self {
		Self {}
	}
}

impl sqpack::Resource for Version {
	fn path_metadata(&self, path: &str) -> Option<(u8, u8)> {
		let split = path.split('/').take(2).collect::<Vec<_>>();

		match split[..] {
			[path_category, path_repository] => Some((
				REPOSITORIES
					.iter()
					.position(|repository| repository == &path_repository)
					.unwrap_or(0)
					.try_into()
					.unwrap(),
				CATEGORIES
					.iter()
					.position(|category| category == &Some(path_category))?
					.try_into()
					.unwrap(),
			)),
			_ => None,
		}
	}

	fn version(&self, repository: u8) -> Result<String> {
		todo!("version({repository})")
	}

	type Index = io::Empty;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		todo!("index({repository}, {category}, {chunk})")
	}

	type Index2 = io::Empty;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2> {
		todo!("index2({repository}, {category}, {chunk})")
	}

	type File = io::Empty;
	fn file(&self, repository: u8, category: u8, location: sqpack::Location) -> Result<Self::File> {
		todo!("file({repository}, {category}, {location:?})")
	}
}
