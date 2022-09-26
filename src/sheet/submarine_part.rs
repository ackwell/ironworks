use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SubmarinePart {
    fn name() -> String {
        "SubmarinePart".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SubmarinePart::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SubmarinePart {
    pub r#slot: u8,
    pub r#rank: u8,
    pub r#components: u8,
    pub r#surveillance: i16,
    pub r#retrieval: i16,
    pub r#speed: i16,
    pub r#range: i16,
    pub r#favor: i16,
    pub r#class: u16,
    pub r#repair_materials: u8,
}
impl SubmarinePart {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#slot: row.field(0usize + offset)?.into_u8()?,
            r#rank: row.field(1usize + offset)?.into_u8()?,
            r#components: row.field(2usize + offset)?.into_u8()?,
            r#surveillance: row.field(3usize + offset)?.into_i16()?,
            r#retrieval: row.field(4usize + offset)?.into_i16()?,
            r#speed: row.field(5usize + offset)?.into_i16()?,
            r#range: row.field(6usize + offset)?.into_i16()?,
            r#favor: row.field(7usize + offset)?.into_i16()?,
            r#class: row.field(8usize + offset)?.into_u16()?,
            r#repair_materials: row.field(9usize + offset)?.into_u8()?,
        })
    }
}
