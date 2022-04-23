use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for Guide {
    fn name() -> String {
        "Guide".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Guide::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Guide {
    pub r#guide_title: u16,
    pub r#guide_page: u16,
}
impl Guide {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#guide_title: row.field(0usize + offset)?.into_u16()?,
            r#guide_page: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
