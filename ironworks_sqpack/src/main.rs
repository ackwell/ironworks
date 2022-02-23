// This file exists as a temporary runner only

mod sqpack;

use std::{collections::HashMap, error::Error, path::Path};

use crate::sqpack::SqPack;

fn main() -> Result<(), Box<dyn Error>> {
	let repositories = HashMap::from([
		("ffxiv", Path::new("/mnt/c/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/ffxiv")),
	]);

	let categories = HashMap::from([("exd", 0x0A)]);

	let sqpack = SqPack {
		repositories,
		categories,
		default_repository: "ffxiv",
	};

	println!("sqpack: {:?}", sqpack);

	sqpack.temp_test("exd/root.exl")?;

	return Ok(());
}
