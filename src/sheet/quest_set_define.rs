use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct QuestSetDefine {
    pub r#unknown0: u32,
}
impl QuestSetDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
