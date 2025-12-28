use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CompanyCraftSequence {
    fn name() -> String {
        "CompanyCraftSequence".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftSequence::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftSequence {
    pub r#result_item: i32,
    pub r#category: i32,
    pub r#company_craft_draft_category: i32,
    pub r#company_craft_type: i32,
    pub r#company_craft_draft: i32,
    pub r#company_craft_part: Vec<u16>,
    pub r#order: u32,
}
impl CompanyCraftSequence {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#result_item: row.field(0usize + offset)?.into_i32()?,
            r#category: row.field(1usize + offset)?.into_i32()?,
            r#company_craft_draft_category: row.field(2usize + offset)?.into_i32()?,
            r#company_craft_type: row.field(3usize + offset)?.into_i32()?,
            r#company_craft_draft: row.field(4usize + offset)?.into_i32()?,
            r#company_craft_part: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u16()?) },
            )?,
            r#order: row.field(13usize + offset)?.into_u32()?,
        })
    }
}
