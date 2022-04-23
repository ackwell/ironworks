use std::io::{Read, Seek};

use crate::error::Result;

/// Resource adapter to fetch data on request for an Excel instance.
pub trait Resource {
	/// The version string of the excel resource data.
	fn version(&self) -> Result<String>;

	/// The type of a sheet list resource.
	type List: Read + Seek;
	/// Fetches the sheet list resource.
	fn list(&self) -> Result<Self::List>;

	/// The type of a sheet header resource.
	type Header: Read + Seek;
	/// Fetches the specified sheet's header resource.
	fn header(&self, sheet: &str) -> Result<Self::Header>;

	/// The type of a page resource.
	type Page: Read + Seek;
	/// Fetches the specified page resource of a sheet.
	fn page(&self, sheet: &str, start_id: u32, language_id: u8) -> Result<Self::Page>;
}
