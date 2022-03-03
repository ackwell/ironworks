// This file exists as a temporary runner only

#![allow(clippy::needless_return)]

mod crc;
mod dat_reader;
mod error;
mod file_struct;
mod index;
mod sqpack;
mod utility;

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

	let file_buffer = sqpack.read_file("exd/root.exl")?;

	let exlt = String::from_utf8(file_buffer).unwrap();

	println!("EXLT: {}", exlt);

	return Ok(());
}
