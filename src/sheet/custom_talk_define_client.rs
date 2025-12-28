use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CustomTalkDefineClient {
    fn name() -> String {
        "CustomTalkDefineClient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CustomTalkDefineClient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CustomTalkDefineClient {
    pub r#unknown0: SeString,
    pub r#unknown1: u32,
}
impl CustomTalkDefineClient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
