use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
