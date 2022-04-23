use crate::error::PopulateError;
use std::vec::Vec;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for StanceChange {
    fn name() -> String {
        "StanceChange".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(StanceChange::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct StanceChange {
    pub r#unknown0: u16,
    pub r#action: Vec<u16>,
}
impl StanceChange {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#action: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
