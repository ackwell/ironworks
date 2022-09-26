use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for QuestEffect {
    fn name() -> String {
        "QuestEffect".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestEffect::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestEffect {}
impl QuestEffect {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
