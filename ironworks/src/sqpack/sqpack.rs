use std::io::{Read, Seek};

trait Resource {
	fn parse_metadata(path: &str) -> (&str, &str);

	type Index: Read + Seek;
	fn index(repository: &str, category: &str, chunk: u8) -> Self::Index;

	type Index2: Read + Seek;
	fn index2(repository: &str, category: &str, chunk: u8) -> Self::Index2;

	type Dat: Read + Seek;
	fn dat(repository: &str, category: &str, chunk: u8) -> Self::Dat;
}

// this is more "ffxiv file system resource" really. where does it go?
struct FileSystemResource {}

impl Resource for FileSystemResource {
	fn parse_metadata(path: &str) -> (&str, &str) {
		("", "")
	}

	type Index = std::io::Empty;
	fn index(repository: &str, category: &str, chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Index2 = std::io::Empty;
	fn index2(repository: &str, category: &str, chunk: u8) -> Self::Index {
		std::io::empty()
	}

	type Dat = std::io::Empty;
	fn dat(repository: &str, category: &str, chunk: u8) -> Self::Index {
		std::io::empty()
	}
}
