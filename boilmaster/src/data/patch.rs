use std::{
	collections::{hash_map::Entry, HashMap},
	fs,
	io::{self, Write},
	path::{Path, PathBuf},
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, Mutex,
	},
};

use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
use serde::Deserialize;
use tokio::sync::{Notify, Semaphore};

use crate::version::Patch;

#[derive(Debug, Deserialize)]
pub struct Config {
	directory: RelativePathBuf,
	concurrency: usize,
}

pub struct Patcher {
	directory: PathBuf,
	semaphore: Semaphore,
	client: reqwest::Client,
	known_patches: Mutex<HashMap<PathBuf, Arc<(AtomicBool, Notify)>>>,
}

impl Patcher {
	pub fn new(config: Config) -> Self {
		Self {
			directory: config.directory.relative(),
			semaphore: Semaphore::new(config.concurrency),
			client: reqwest::Client::new(),
			known_patches: Default::default(),
		}
	}

	// TODO: proper error type
	pub async fn patch_paths(
		&self,
		repository_name: &str,
		patches: &[Patch],
	) -> Result<HashMap<String, PathBuf>> {
		// TODO: This seems silly to run on a regular basis given it'll only actually do something like 4 times ever.
		let repository_directory = self.directory.join(repository_name);
		fs::create_dir_all(&repository_directory)?;

		// Ensure each of the paths in the list exists.
		let pending_patches = patches.iter().map(|patch| {
			let path = repository_directory.join(&patch.name);
			async move {
				self.ensure_patch_exists(patch, &path)
					.await
					.map(|_| (patch, path))
			}
		});

		let checked_patches = try_join_all(pending_patches).await?;

		// Build the final path mapping.
		let path_map = checked_patches
			.into_iter()
			.map(|(patch, path)| (patch.name.clone(), path))
			.collect::<HashMap<_, _>>();

		Ok(path_map)
	}

	async fn ensure_patch_exists(&self, patch: &Patch, path: &Path) -> Result<()> {
		// Grab the current state of this patch from known patch store, taking
		// responsibility for checking the patch if it is not yet known.
		let (occupied, value) = match self
			.known_patches
			.lock()
			.expect("poisoned")
			.entry(path.to_path_buf())
		{
			Entry::Occupied(entry) => (true, entry.get().clone()),
			Entry::Vacant(entry) => (
				false,
				entry
					.insert(Arc::new((AtomicBool::new(false), Notify::new())))
					.clone(),
			),
		};

		let (ready, notify) = &*value;

		// If the patch is aleady known, or being checked, wait on the resolution
		// from the responsible task (if any).
		if occupied {
			// TODO: I'm not sure that this loop is actually relevant, but this logic is mostly 1:1 from a std convar impl.
			while !ready.load(Ordering::SeqCst) {
				notify.notified().await;
			}
			return Ok(());
		}

		// This task is responsible for checking the patch file - check it and download if required.
		if self.should_fetch_patch(patch, path)? {
			// TODO: most of this can be inlined into this function i'd assume - or at least brought into the struct
			download_patch(&self.client, patch, path, &self.semaphore).await?;
		}

		// The patch is ready by this point, mark it as such and notify any other tasks waiting on it.
		ready.store(true, Ordering::SeqCst);
		notify.notify_waiters();

		Ok(())
	}

	fn should_fetch_patch(&self, patch: &Patch, path: &Path) -> Result<bool> {
		let metadata = match path.metadata() {
			// NotFound implies the patch doesn't exist, and should be downloaded.
			Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(true),
			other => other?,
		};

		// If there's a size mismatch, it should be re-downloaded (likely a partial completion).
		let size_matches = metadata.len() == patch.size;

		if !size_matches {
			tracing::warn!(
				patch = %patch.name,
				expected = patch.size,
				got = metadata.len(),
				"size mismatch, will re-fetch"
			);
		}

		// TODO: I _imagine_ this should probably actually do something about non-file paths, but for now it'll fail out somewhere else.
		Ok(!(path.is_file() && size_matches))
	}
}

async fn download_patch(
	client: &reqwest::Client,
	patch: &Patch,
	target_path: &Path,
	semaphore: &Semaphore,
) -> Result<()> {
	let permit = semaphore.acquire().await.unwrap();

	tracing::info!("downloading patch {}", patch.name);

	// Create the target file before opening any connections.
	let mut target_file = fs::File::create(target_path)?;

	// Initiate a request to the patch file
	let mut response = client.get(&patch.url).send().await?;
	let content_length = response.content_length().ok_or_else(|| {
		anyhow::anyhow!("Could not find patch content length for {}.", patch.name)
	})?;

	// Stream the file to disk.
	let mut position = 0;
	let mut last_report = 0.;
	while let Some(chunk) = response.chunk().await? {
		// this is probably blocking - is it worth doing some of this on a spawn_blocking?
		target_file.write_all(&chunk)?;

		position += u64::try_from(chunk.len()).unwrap();
		let report_pos = f64::round((position as f64 / content_length as f64) * 20.) * 5.;
		if report_pos > last_report {
			tracing::debug!(
				"{}: {position}/{content_length} ({report_pos}%)",
				patch.name
			);
			last_report = report_pos;
		}
	}

	drop(permit);

	Ok(())
}
