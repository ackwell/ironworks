use std::marker::PhantomData;

use super::{excel::Excel, resource::Resource};

/// Options for the root Excel database.
#[derive(Debug)]
pub struct ExcelOptions<R> {
	pub(super) language: Option<u8>,

	// NOTE: This is used to allow the resource generic type to be inferred when
	// built via Excel::with.
	_r: PhantomData<R>,
}

impl<R: Resource> ExcelOptions<R> {
	/// Set the default language of the database
	pub fn language(&mut self, language: impl Into<u8>) -> &mut Self {
		self.language = Some(language.into());
		self
	}

	/// Build the configured Excel database.
	pub fn build(&self, resource: R) -> Excel<R> {
		Excel::with_options(resource, self)
	}
}

impl<R> Default for ExcelOptions<R> {
	fn default() -> Self {
		Self {
			language: None,
			_r: Default::default(),
		}
	}
}
