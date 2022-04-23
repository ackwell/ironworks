use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for QuestEffectDefine {
    fn name() -> String {
        "QuestEffectDefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestEffectDefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestEffectDefine {
    pub r#effect: u16,
}
impl QuestEffectDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#effect: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
