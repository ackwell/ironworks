use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MapTransientPvPMap {
    fn name() -> String {
        "MapTransientPvPMap".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MapTransientPvPMap::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MapTransientPvPMap {}
impl MapTransientPvPMap {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
