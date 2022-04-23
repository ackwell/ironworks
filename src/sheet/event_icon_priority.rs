use crate::utility::read_array;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for EventIconPriority {
    fn name() -> String {
        "EventIconPriority".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventIconPriority::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventIconPriority {
    pub r#icon: Vec<u32>,
}
impl EventIconPriority {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: read_array(
                offset,
                19usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
