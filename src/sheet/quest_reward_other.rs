use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for QuestRewardOther {
    fn name() -> String {
        "QuestRewardOther".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRewardOther::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRewardOther {
    pub r#icon: u32,
    pub r#name: SeString,
}
impl QuestRewardOther {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
