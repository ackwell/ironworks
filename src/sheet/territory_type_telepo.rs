use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for TerritoryTypeTelepo {
    fn name() -> String {
        "TerritoryTypeTelepo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TerritoryTypeTelepo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TerritoryTypeTelepo {}
impl TerritoryTypeTelepo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
