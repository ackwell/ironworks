use core::fmt;
use std::hash::{Hash, Hasher};

use ironworks::excel::Sheet;
use seahash::SeaHasher;

use crate::{search::error::Result, version::VersionKey};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SheetKey(u64);

impl SheetKey {
	pub fn from_sheet_version(version: VersionKey, sheet_name: &str) -> Self {
		let mut hasher = SeaHasher::new();
		version.hash(&mut hasher);
		sheet_name.hash(&mut hasher);
		SheetKey(hasher.finish())
	}
}

impl From<SheetKey> for u64 {
	fn from(value: SheetKey) -> Self {
		value.0
	}
}

impl From<u64> for SheetKey {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl fmt::Display for SheetKey {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_fmt(format_args!("{:016x}", self.0))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexKey(u64);

impl IndexKey {
	pub fn try_from_sheet(sheet: &Sheet<String>) -> Result<Self> {
		// TODO: consider using fixed seeds?
		let mut hasher = SeaHasher::new();
		sheet.kind()?.hash(&mut hasher);

		let mut languages = sheet.languages()?;
		languages.sort_by_key(|language| u8::from(*language));
		languages.hash(&mut hasher);

		// TODO: this encodes the offsets of the columns as well as their kind (and position due to the vec) - technically the actual offset is irrelevant, so would be good to ignore it, but doing so would require decoupling column names from offsets, which I can't do without changes to a lot of stuff in search query resolution. i'm not convinced that different offset layouts for the same structure are going to be common enough to bother.
		let mut columns = sheet.columns()?;
		columns.sort_by_key(|column| column.offset());
		columns.hash(&mut hasher);

		Ok(IndexKey(hasher.finish()))
	}
}

impl fmt::Display for IndexKey {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_fmt(format_args!("{:016x}", self.0))
	}
}
