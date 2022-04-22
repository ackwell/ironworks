use ironworks::excel::{Row, SheetMetadata};
use std::marker::PhantomData;

use crate::PopulateError;

pub trait MetadataAdapter {
	fn name() -> String;
	fn populate(row: &Row) -> Result<Self, PopulateError>
	where
		Self: Sized;
}

/// Retrieve a `SheetMetadata` value for the sheet struct type `S`.
pub fn metadata<S>() -> SheetType<S> {
	SheetType {
		_sheet: PhantomData::<S>::default(),
	}
}

pub struct SheetType<S> {
	_sheet: PhantomData<S>,
}

impl<S: MetadataAdapter> SheetMetadata for SheetType<S> {
	fn name(&self) -> String {
		S::name()
	}

	type Row = S;
	type Error = PopulateError;
	fn populate_row(&self, row: Row) -> Result<Self::Row, Self::Error> {
		S::populate(&row)
	}
}
