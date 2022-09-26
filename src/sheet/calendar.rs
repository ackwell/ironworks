use std::vec::Vec;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for Calendar {
    fn name() -> String {
        "Calendar".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Calendar::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Calendar {
    pub r#month: Vec<u8>,
    pub r#day: Vec<u8>,
}
impl Calendar {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#month: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
            r#day: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(32usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
