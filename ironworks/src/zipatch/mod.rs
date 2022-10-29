// TEMP
#![allow(missing_docs)]

use std::{fs, io::BufReader};

use crate::file::{
	patch::{self, FileOperationCommand},
	File,
};

pub fn temp_test() {
	// for now, i'm just hardcoding a lookup for 0a (exd)'s index. this should likely, at _minimum_, store the metadata for all found indices in a cache structure of some kind
	let target = "sqpack/ffxiv/0a0000.win32.index";
	let out = PATCH_LIST
		.iter()
		// atm patch list is in oldest-first, and we want to get the newest copy of a file
		.rev()
		// hardcoding because lazy
		.map(|filename| format!("/mnt/c/Users/ackwell/code/xiv/patches/game/4e9a232b/{filename}"))
		// get the zipatch struct for each file
		.map(|filepath| {
			println!("checking {filepath}");
			let file = fs::File::open(filepath).expect("TODO");
			let buf = BufReader::new(file);
			patch::ZiPatch::read(buf).expect("TODO")
		})
		// operating on full patches at a time - this makes the (safe?) assumption that the granularity of a game _version_ is at _minimum_ one patch.
		.find_map(|zipatch| {
			zipatch.chunks().find_map(|chunk| {
				let chunk = chunk.expect("TODO");
				match &chunk {
					patch::Chunk::SqPack(patch::SqPackChunk::FileOperation(
						command @ FileOperationCommand { .. },
					)) if matches!(command.operation(), patch::FileOperation::AddFile(_))
						&& command.path().to_string() == target =>
					{
						Some(chunk)
					}
					_ => None,
				}
			})
		});

	println!("found: {out:#?}")
}

// Temp listing because i'm being lazy and don't want to handle the ordering side of things right now
const PATCH_LIST: &[&str] = &[
	"H2017.06.06.0000.0001a.patch",
	"H2017.06.06.0000.0001b.patch",
	"H2017.06.06.0000.0001c.patch",
	"H2017.06.06.0000.0001d.patch",
	"H2017.06.06.0000.0001e.patch",
	"H2017.06.06.0000.0001f.patch",
	"H2017.06.06.0000.0001g.patch",
	"H2017.06.06.0000.0001h.patch",
	"H2017.06.06.0000.0001i.patch",
	"H2017.06.06.0000.0001j.patch",
	"H2017.06.06.0000.0001k.patch",
	"H2017.06.06.0000.0001l.patch",
	"H2017.06.06.0000.0001m.patch",
	"H2017.06.06.0000.0001n.patch",
	"D2017.07.11.0000.0001.patch",
	"D2017.09.24.0000.0001.patch",
	"D2017.10.11.0000.0001.patch",
	"D2017.10.31.0000.0001.patch",
	"D2017.11.24.0000.0001.patch",
	"D2018.01.12.0000.0001.patch",
	"D2018.02.09.0000.0001.patch",
	"D2018.04.27.0000.0001.patch",
	"D2018.05.26.0000.0001.patch",
	"D2018.06.19.0000.0001.patch",
	"D2018.07.18.0000.0001.patch",
	"D2018.09.05.0000.0001.patch",
	"D2018.10.19.0000.0001.patch",
	"D2018.12.15.0000.0001.patch",
	"D2019.01.26.0000.0001.patch",
	"D2019.03.12.0000.0001.patch",
	"D2019.03.15.0000.0001.patch",
	"D2019.04.16.0000.0001.patch",
	"D2019.05.09.0000.0001.patch",
	"D2019.05.29.0000.0000.patch",
	"D2019.05.29.0001.0000.patch",
	"D2019.05.31.0000.0001.patch",
	"D2019.06.07.0000.0001.patch",
	"D2019.07.09.0000.0001.patch",
	"D2019.07.10.0001.0001.patch",
	"D2019.10.11.0000.0001.patch",
	"D2019.10.16.0000.0001.patch",
	"D2019.11.02.0000.0001.patch",
	"D2019.11.19.0000.0001.patch",
	"D2019.12.19.0000.0001.patch",
	"D2020.01.31.0000.0000.patch",
	"D2020.01.31.0001.0000.patch",
	"D2020.02.27.0000.0001.patch",
	"D2020.03.24.0000.0001.patch",
	"D2020.03.27.0000.0001.patch",
	"D2020.07.21.0000.0000.patch",
	"D2020.09.15.0000.0001.patch",
	"D2020.10.06.0000.0001.patch",
	"D2020.11.24.0000.0001.patch",
	"D2020.12.15.0000.0001.patch",
	"D2021.01.14.0000.0001.patch",
	"D2021.02.10.0000.0001.patch",
	"D2021.03.26.0000.0000.patch",
	"D2021.03.26.0001.0000.patch",
	"D2021.03.26.0002.0000.patch",
	"D2021.03.26.0003.0000.patch",
	"D2021.03.30.0000.0000.patch",
	"D2021.04.29.0000.0001.patch",
	"D2021.05.07.0000.0001.patch",
	"D2021.06.16.0000.0001.patch",
	"D2021.08.17.0000.0001.patch",
	"D2021.11.17.0000.0000.patch",
	"D2021.11.17.0001.0000.patch",
	"D2021.11.17.0002.0000.patch",
	"D2021.11.20.0000.0001.patch",
	"D2021.11.27.0000.0001.patch",
	"D2021.11.28.0000.0000.patch",
	"D2021.12.15.0000.0001.patch",
	"D2021.12.16.0000.0000.patch",
	"D2021.12.23.0000.0000.patch",
	"D2021.12.24.0000.0000.patch",
	"D2022.01.19.0000.0001.patch",
	"D2022.01.25.0000.0000.patch",
	"D2022.03.01.0000.0000.patch",
	"D2022.03.25.0000.0000.patch",
	"D2022.03.25.0001.0000.patch",
	"D2022.03.31.0000.0001.patch",
	"D2022.04.03.0000.0001.patch",
	"D2022.04.06.0000.0001.patch",
	"D2022.04.07.0000.0000.patch",
	"D2022.04.19.0000.0000.patch",
	"D2022.04.20.0000.0000.patch",
	"D2022.05.18.0000.0000.patch",
	"D2022.05.19.0000.0000.patch",
	"D2022.05.26.0000.0000.patch",
	"D2022.05.27.0000.0000.patch",
	"D2022.06.17.0000.0001.patch",
	"D2022.06.21.0000.0000.patch",
	"D2022.07.08.0000.0000.patch",
	"D2022.08.05.0000.0000.patch",
	"D2022.08.05.0001.0000.patch",
	"D2022.08.10.0000.0001.patch",
	"D2022.08.12.0000.0001.patch",
	"D2022.08.16.0000.0001.patch",
	"D2022.08.17.0000.0000.patch",
	"D2022.08.25.0000.0000.patch",
	"D2022.09.06.0000.0000.patch",
	"D2022.09.07.0000.0000.patch",
	"D2022.09.29.0000.0000.patch",
	"D2022.10.01.0000.0001.patch",
	"D2022.10.04.0000.0000.patch",
	"D2022.10.05.0000.0000.patch",
];
