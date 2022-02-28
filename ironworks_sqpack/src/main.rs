// This file exists as a temporary runner only

#![allow(clippy::needless_return)]

mod crc;
mod dat_reader;
mod errors;
mod file_structs;
mod sqpack;

use std::{error::Error, path::PathBuf};

use sqpack::{Category, Repository, SqPack};

fn main() -> Result<(), Box<dyn Error>> {
	let sqpack = SqPack::new(String::from("ffxiv"), [
		Repository {
			id: 0,
			name: String::from("ffxiv"),
			path: PathBuf::from("/mnt/c/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/ffxiv")
		}
	], [
		Category {
			id: 0x0A,
			name: String::from("exd"),
		}
	]);

	sqpack.temp_test("exd/root.exl")?;

	return Ok(());
}
