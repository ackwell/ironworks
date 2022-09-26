use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ClassJobActionSort {
    fn name() -> String {
        "ClassJobActionSort".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ClassJobActionSort::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ClassJobActionSort {}
impl ClassJobActionSort {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
