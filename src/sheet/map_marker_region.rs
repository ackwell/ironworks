use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for MapMarkerRegion {
    fn name() -> String {
        "MapMarkerRegion".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MapMarkerRegion::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MapMarkerRegion {
    pub r#unknown0: u8,
    pub r#x: i16,
}
impl MapMarkerRegion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#x: row.field(1usize + offset)?.into_i16()?,
        })
    }
}
