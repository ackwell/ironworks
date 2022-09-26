use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for OpenContent {
    fn name() -> String {
        "OpenContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OpenContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OpenContent {
    pub r#content: Vec<u16>,
    pub r#candidate_name: Vec<u32>,
}
impl OpenContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content: read_array(
                offset,
                16usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#candidate_name: read_array(
                offset,
                16usize,
                1usize,
                |offset| { Result::Ok(row.field(16usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
