use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for UDS_Event {
    fn name() -> String {
        "UDS_Event".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(UDS_Event::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct UDS_Event {
    pub r#text: SeString,
    pub r#type: SeString,
    pub r#property: Vec<i32>,
}
impl UDS_Event {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#type: row.field(1usize + offset)?.into_string()?,
            r#property: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
