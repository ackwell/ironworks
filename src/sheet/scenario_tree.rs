use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
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
}
impl ScenarioTree {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#addon: row.field(2usize + offset)?.into_u32()?,
            r#quest_chapter: row.field(3usize + offset)?.into_u32()?,
            r#name: row.field(4usize + offset)?.into_string()?,
        })
    }
}
