use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for ScenarioTreeTipsClassQuest {
    fn name() -> String {
        "ScenarioTreeTipsClassQuest".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ScenarioTreeTipsClassQuest::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ScenarioTreeTipsClassQuest {
    pub r#quest: u32,
    pub r#required_level: u16,
    pub r#required_expansion: u8,
    pub r#required_quest: u32,
}
impl ScenarioTreeTipsClassQuest {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#required_level: row.field(1usize + offset)?.into_u16()?,
            r#required_expansion: row.field(2usize + offset)?.into_u8()?,
            r#required_quest: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
