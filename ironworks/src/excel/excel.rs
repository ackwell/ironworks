use crate::error::Result;


/// An excel database.
#[derive(Debug)]
pub struct Excel<R> {
	resource: R,
}

impl<R: Resource> Excel<R> {
	/// Build a representation of an Excel database.
	pub fn new(resource: R) -> Self {
		Self { resource }
	}

	/// Fetch a sheet from the database.
	pub fn sheet(&self, sheet: &str) -> Result<()> {
		Ok(())
	}
}
