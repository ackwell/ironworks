use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown3: i8,
    pub r#unknown4: i8,
    pub r#unknown5: u8,
}
impl ContentTalkParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#param: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#test_action: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
