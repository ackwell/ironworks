use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for ArrayEventHandler {
    fn name() -> String {
        "ArrayEventHandler".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ArrayEventHandler::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ArrayEventHandler {
    pub r#data: Vec<u32>,
}
impl ArrayEventHandler {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#data: read_array(
                offset,
                16usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
