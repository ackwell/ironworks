//! Tools for working with the Excel database format.

mod excel;
mod field;
mod iterator;
mod language;
mod metadata;
pub mod path;
mod row;
mod sheet;

pub use {
	excel::Excel,
	field::Field,
	iterator::SheetIterator,
	language::Language,
	metadata::SheetMetadata,
	row::{ColumnSpecifier, Row},
	sheet::{RowOptions, Sheet},
};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<ColumnSpecifier>();
		assert_send::<Excel>();
		assert_send::<Field>();
		assert_send::<Language>();
		assert_send::<Row>();
		assert_send::<RowOptions>();
		assert_send::<Sheet<()>>();
		assert_send::<SheetIterator<()>>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<ColumnSpecifier>();
		assert_sync::<Excel>();
		assert_sync::<Field>();
		assert_sync::<Language>();
		assert_sync::<Row>();
		assert_sync::<RowOptions>();
		assert_sync::<Sheet<()>>();
		assert_sync::<SheetIterator<()>>();
	}
}
