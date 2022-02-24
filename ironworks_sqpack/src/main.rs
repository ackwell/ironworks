// This file exists as a temporary runner only

#![allow(clippy::needless_return)]

mod file_structs;
mod sqpack;

use std::{collections::HashMap, error::Error, path::PathBuf};

use crate::sqpack::SqPack;

fn main() -> Result<(), Box<dyn Error>> {
	let repositories = HashMap::from([
		(String::from("ffxiv"), PathBuf::from("/mnt/c/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/ffxiv")),
	]);

	let categories = HashMap::from([(String::from("exd"), 0x0A)]);

	let sqpack = SqPack {
		repositories,
		categories,
		default_repository: String::from("ffxiv"),
	};

	println!("sqpack: {:?}", sqpack);

	sqpack.temp_test("exd/root.exl")?;

	return Ok(());
}
