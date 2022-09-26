use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for QuestDerivedClass {
    fn name() -> String {
        "QuestDerivedClass".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestDerivedClass::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestDerivedClass {
    pub r#class_job: u8,
}
impl QuestDerivedClass {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#class_job: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
