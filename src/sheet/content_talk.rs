use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for ContentTalk {
    fn name() -> String {
        "ContentTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentTalk {
    pub r#content_talk_param: u8,
    pub r#text: SeString,
}
impl ContentTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_talk_param: row.field(0usize + offset)?.into_u8()?,
            r#text: row.field(1usize + offset)?.into_string()?,
        })
    }
}
