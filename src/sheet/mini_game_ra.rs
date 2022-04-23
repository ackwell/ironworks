use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for MiniGameRA {
    fn name() -> String {
        "MiniGameRA".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MiniGameRA::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MiniGameRA {
    pub r#unknown0: i32,
    pub r#icon: i32,
    pub r#image: i32,
    pub r#bgm: i32,
}
impl MiniGameRA {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#image: row.field(2usize + offset)?.into_i32()?,
            r#bgm: row.field(3usize + offset)?.into_i32()?,
        })
    }
}
