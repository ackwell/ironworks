use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for CharaMakeClassEquip {
    fn name() -> String {
        "CharaMakeClassEquip".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaMakeClassEquip::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaMakeClassEquip {
    pub r#helmet: u64,
    pub r#top: u64,
    pub r#glove: u64,
    pub r#down: u64,
    pub r#shoes: u64,
    pub r#weapon: u64,
    pub r#sub_weapon: u64,
    pub r#class: i32,
}
impl CharaMakeClassEquip {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#helmet: row.field(0usize + offset)?.into_u64()?,
            r#top: row.field(1usize + offset)?.into_u64()?,
            r#glove: row.field(2usize + offset)?.into_u64()?,
            r#down: row.field(3usize + offset)?.into_u64()?,
            r#shoes: row.field(4usize + offset)?.into_u64()?,
            r#weapon: row.field(5usize + offset)?.into_u64()?,
            r#sub_weapon: row.field(6usize + offset)?.into_u64()?,
            r#class: row.field(7usize + offset)?.into_i32()?,
        })
    }
}
