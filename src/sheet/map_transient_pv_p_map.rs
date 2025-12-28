use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct MapTransientPvPMap {
    pub r#unknown0: u32,
    pub r#unknown1: u16,
}
impl MapTransientPvPMap {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
