use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for PatchMark {
    fn name() -> String {
        "PatchMark".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PatchMark::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PatchMark {
    pub r#category: i8,
    pub r#sub_category_type: u8,
    pub r#sub_category: u16,
    pub r#unknown3: u8,
    pub r#unknown4: u32,
    pub r#mark_id: u32,
    pub r#version: u8,
    pub r#unknown7: u16,
}
impl PatchMark {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_i8()?,
            r#sub_category_type: row.field(1usize + offset)?.into_u8()?,
            r#sub_category: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#mark_id: row.field(5usize + offset)?.into_u32()?,
            r#version: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
        })
    }
}
