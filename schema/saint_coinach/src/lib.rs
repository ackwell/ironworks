use std::env::current_exe;

use git2::{build::RepoBuilder, Repository};

pub fn test() {
	// todo error handling lmao
	let repository_path = current_exe()
		.unwrap()
		.parent()
		.unwrap()
		.join("__test_coinach_repo");

	let repository = repository_path
		.exists()
		.then(|| Repository::open_bare(&repository_path))
		.unwrap_or_else(|| {
			RepoBuilder::new().bare(true).clone(
				"https://github.com/xivapi/SaintCoinach.git",
				&repository_path,
			)
		});

	match repository {
		Err(e) => panic!("{}", e),
		Ok(repo) => println!("yay:{}", repo.head().unwrap().name().unwrap()),
	}
}
