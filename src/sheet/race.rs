use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for Race {
    fn name() -> String {
        "Race".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Race::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Race {
    pub r#masculine: SeString,
    pub r#feminine: SeString,
    pub r#rsem_body: i32,
    pub r#rsem_hands: i32,
    pub r#rsem_legs: i32,
    pub r#rsem_feet: i32,
    pub r#rsef_body: i32,
    pub r#rsef_hands: i32,
    pub r#rsef_legs: i32,
    pub r#rsef_feet: i32,
    pub r#unknown10: u8,
    pub r#ex_pac: u8,
}
impl Race {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#masculine: row.field(0usize + offset)?.into_string()?,
            r#feminine: row.field(1usize + offset)?.into_string()?,
            r#rsem_body: row.field(2usize + offset)?.into_i32()?,
            r#rsem_hands: row.field(3usize + offset)?.into_i32()?,
            r#rsem_legs: row.field(4usize + offset)?.into_i32()?,
            r#rsem_feet: row.field(5usize + offset)?.into_i32()?,
            r#rsef_body: row.field(6usize + offset)?.into_i32()?,
            r#rsef_hands: row.field(7usize + offset)?.into_i32()?,
            r#rsef_legs: row.field(8usize + offset)?.into_i32()?,
            r#rsef_feet: row.field(9usize + offset)?.into_i32()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#ex_pac: row.field(11usize + offset)?.into_u8()?,
        })
    }
}
