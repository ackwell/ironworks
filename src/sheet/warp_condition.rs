use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for WarpCondition {
    fn name() -> String {
        "WarpCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WarpCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WarpCondition {
    pub r#gil: u16,
    pub r#complete_param: u8,
    pub r#required_quest1: u32,
    pub r#required_quest2: u32,
    pub r#d_required_quest3: u32,
    pub r#required_quest4: u32,
    pub r#quest_reward: u16,
    pub r#class_level: u16,
}
impl WarpCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gil: row.field(0usize + offset)?.into_u16()?,
            r#complete_param: row.field(1usize + offset)?.into_u8()?,
            r#required_quest1: row.field(2usize + offset)?.into_u32()?,
            r#required_quest2: row.field(3usize + offset)?.into_u32()?,
            r#d_required_quest3: row.field(4usize + offset)?.into_u32()?,
            r#required_quest4: row.field(5usize + offset)?.into_u32()?,
            r#quest_reward: row.field(6usize + offset)?.into_u16()?,
            r#class_level: row.field(7usize + offset)?.into_u16()?,
        })
    }
}
