use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for QuestClassJobSupply {
    fn name() -> String {
        "QuestClassJobSupply".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestClassJobSupply::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestClassJobSupply {
    pub r#class_job_category: u8,
    pub r#unknown1: u8,
    pub r#e_npc_resident: u32,
    pub r#item: u32,
    pub r#amount_required: u8,
    pub r#item_hq: bool,
}
impl QuestClassJobSupply {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#class_job_category: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#e_npc_resident: row.field(2usize + offset)?.into_u32()?,
            r#item: row.field(3usize + offset)?.into_u32()?,
            r#amount_required: row.field(4usize + offset)?.into_u8()?,
            r#item_hq: row.field(5usize + offset)?.into_bool()?,
        })
    }
}
