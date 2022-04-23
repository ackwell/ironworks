use ironworks::excel::Row;
use std::result::Result;
use crate::utility::read_array;
use crate::error::PopulateError;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PerformGroup {
    fn name() -> String {
        "PerformGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PerformGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PerformGroup {
    pub r#perform: Vec<u8>,
}
impl PerformGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#perform: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
