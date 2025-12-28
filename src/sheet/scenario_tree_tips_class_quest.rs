use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown4: bool,
    pub r#unknown5: bool,
}
impl ScenarioTreeTipsClassQuest {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#required_level: row.field(1usize + offset)?.into_u16()?,
            r#required_expansion: row.field(2usize + offset)?.into_u8()?,
            r#required_quest: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
        })
    }
}
