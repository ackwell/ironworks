use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::vec::Vec;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CompanyCraftDraft {
    fn name() -> String {
        "CompanyCraftDraft".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftDraft::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftDraft_RequiredItem {
    pub r#required_item: i32,
    pub r#required_item_count: u8,
}
impl CompanyCraftDraft_RequiredItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#required_item: row.field(2usize + offset)?.into_i32()?,
            r#required_item_count: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct CompanyCraftDraft {
    pub r#name: SeString,
    pub r#company_craft_draft_category: u8,
    pub r#required_item: Vec<CompanyCraftDraft_RequiredItem>,
    pub r#order: u32,
}
impl CompanyCraftDraft {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#company_craft_draft_category: row.field(1usize + offset)?.into_u8()?,
            r#required_item: read_array(
                offset,
                3usize,
                2usize,
                |offset| {
                    Result::Ok(CompanyCraftDraft_RequiredItem::populate(row, offset)?)
                },
            )?,
            r#order: row.field(8usize + offset)?.into_u32()?,
        })
    }
}
