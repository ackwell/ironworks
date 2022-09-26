use ironworks::excel::Row;
use std::vec::Vec;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
impl MetadataAdapter for RacingChocoboItem {
    fn name() -> String {
        "RacingChocoboItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RacingChocoboItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RacingChocoboItem {
    pub r#item: i32,
    pub r#category: u8,
    pub r#param: Vec<u8>,
}
impl RacingChocoboItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#category: row.field(1usize + offset)?.into_u8()?,
            r#param: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
