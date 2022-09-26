use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentTalkParam {
    fn name() -> String {
        "ContentTalkParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentTalkParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentTalkParam {
    pub r#param: bool,
    pub r#unknown1: u8,
    pub r#test_action: u32,
}
impl ContentTalkParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#param: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#test_action: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
