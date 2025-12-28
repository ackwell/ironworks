use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct MapExclusive {
    pub r#unknown0: u8,
    pub r#unknown1: u16,
}
impl MapExclusive {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
