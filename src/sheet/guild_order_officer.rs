use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GuildOrderOfficer {
    fn name() -> String {
        "GuildOrderOfficer".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuildOrderOfficer::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuildOrderOfficer {}
impl GuildOrderOfficer {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
