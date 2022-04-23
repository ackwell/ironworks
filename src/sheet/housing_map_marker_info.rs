use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for HousingMapMarkerInfo {
    fn name() -> String {
        "HousingMapMarkerInfo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingMapMarkerInfo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingMapMarkerInfo {
    pub r#x: f32,
    pub r#y: f32,
    pub r#z: f32,
    pub r#unknown3: f32,
    pub r#map: u16,
}
impl HousingMapMarkerInfo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#x: row.field(0usize + offset)?.into_f32()?,
            r#y: row.field(1usize + offset)?.into_f32()?,
            r#z: row.field(2usize + offset)?.into_f32()?,
            r#unknown3: row.field(3usize + offset)?.into_f32()?,
            r#map: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
