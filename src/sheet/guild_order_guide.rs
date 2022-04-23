use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for GuildOrderGuide {
    fn name() -> String {
        "GuildOrderGuide".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuildOrderGuide::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuildOrderGuide {}
impl GuildOrderGuide {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
