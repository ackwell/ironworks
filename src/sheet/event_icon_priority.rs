use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
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
    pub r#unknown19: u32,
    pub r#unknown20: u32,
    pub r#unknown21: u32,
    pub r#unknown22: u32,
    pub r#unknown23: u32,
    pub r#unknown24: u32,
    pub r#unknown25: u32,
    pub r#unknown26: u32,
    pub r#unknown27: u32,
    pub r#unknown28: u32,
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
            r#unknown19: row.field(19usize + offset)?.into_u32()?,
            r#unknown20: row.field(20usize + offset)?.into_u32()?,
            r#unknown21: row.field(21usize + offset)?.into_u32()?,
            r#unknown22: row.field(22usize + offset)?.into_u32()?,
            r#unknown23: row.field(23usize + offset)?.into_u32()?,
            r#unknown24: row.field(24usize + offset)?.into_u32()?,
            r#unknown25: row.field(25usize + offset)?.into_u32()?,
            r#unknown26: row.field(26usize + offset)?.into_u32()?,
            r#unknown27: row.field(27usize + offset)?.into_u32()?,
            r#unknown28: row.field(28usize + offset)?.into_u32()?,
        })
    }
}
