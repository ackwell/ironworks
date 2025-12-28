use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for RecommendContents {
    fn name() -> String {
        "RecommendContents".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RecommendContents::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RecommendContents {
    pub r#level: i32,
    pub r#class_job: u8,
    pub r#min_level: u8,
    pub r#max_level: u8,
}
impl RecommendContents {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level: row.field(0usize + offset)?.into_i32()?,
            r#class_job: row.field(1usize + offset)?.into_u8()?,
            r#min_level: row.field(2usize + offset)?.into_u8()?,
            r#max_level: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
