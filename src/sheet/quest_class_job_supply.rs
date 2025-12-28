use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#unknown8: u32,
    pub r#unknown9: u16,
    pub r#unknown10: u8,
    pub r#unknown11: u8,
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
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
        })
    }
}
