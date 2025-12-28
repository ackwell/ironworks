use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CSBonusMission {
    fn name() -> String {
        "CSBonusMission".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CSBonusMission::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CSBonusMission {
    pub r#content: Vec<u16>,
}
impl CSBonusMission {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
