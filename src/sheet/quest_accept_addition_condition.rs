use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
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
    pub r#requirement0: u32,
    pub r#requirement1: u32,
}
impl QuestAcceptAdditionCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#requirement0: row.field(0usize + offset)?.into_u32()?,
            r#requirement1: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
