use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for QuestAcceptAdditionCondition {
    fn name() -> String {
        "QuestAcceptAdditionCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestAcceptAdditionCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestAcceptAdditionCondition {
    pub r#requirement0: bool,
    pub r#requirement1: u32,
    pub r#unknown2: u32,
    pub r#unknown3: u32,
    pub r#unknown4: bool,
}
impl QuestAcceptAdditionCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#requirement0: row.field(0usize + offset)?.into_bool()?,
            r#requirement1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
