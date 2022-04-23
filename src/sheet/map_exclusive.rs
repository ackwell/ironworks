use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for MapExclusive {
    fn name() -> String {
        "MapExclusive".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MapExclusive::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MapExclusive {}
impl MapExclusive {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
