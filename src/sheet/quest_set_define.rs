use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for QuestSetDefine {
    fn name() -> String {
        "QuestSetDefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestSetDefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestSetDefine {}
impl QuestSetDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
