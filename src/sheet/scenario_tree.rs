use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for ScenarioTree {
    fn name() -> String {
        "ScenarioTree".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ScenarioTree::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ScenarioTree {
    pub r#type: u8,
    pub r#unknown1: u16,
    pub r#addon: u32,
    pub r#quest_chapter: u32,
    pub r#name: SeString,
    pub r#unknown5: i32,
    pub r#unknown6: u32,
}
impl ScenarioTree {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#addon: row.field(2usize + offset)?.into_u32()?,
            r#quest_chapter: row.field(3usize + offset)?.into_u32()?,
            r#name: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_i32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
        })
    }
}
