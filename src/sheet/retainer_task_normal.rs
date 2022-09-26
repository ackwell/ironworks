use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for RetainerTaskNormal {
    fn name() -> String {
        "RetainerTaskNormal".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerTaskNormal::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerTaskNormal {
    pub r#item: i32,
    pub r#quantity0: u8,
    pub r#quantity1: u8,
    pub r#quantity2: u8,
    pub r#quantity3: u8,
    pub r#quantity4: u8,
    pub r#gathering_log: i16,
    pub r#fishing_log: i16,
}
impl RetainerTaskNormal {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#quantity0: row.field(1usize + offset)?.into_u8()?,
            r#quantity1: row.field(2usize + offset)?.into_u8()?,
            r#quantity2: row.field(3usize + offset)?.into_u8()?,
            r#quantity3: row.field(4usize + offset)?.into_u8()?,
            r#quantity4: row.field(5usize + offset)?.into_u8()?,
            r#gathering_log: row.field(6usize + offset)?.into_i16()?,
            r#fishing_log: row.field(7usize + offset)?.into_i16()?,
        })
    }
}
