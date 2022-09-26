use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
impl MetadataAdapter for ContentNpcTalk {
    fn name() -> String {
        "ContentNpcTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentNpcTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentNpcTalk {
    pub r#type: i32,
    pub r#content_talk: Vec<u32>,
}
impl ContentNpcTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_i32()?,
            r#content_talk: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
