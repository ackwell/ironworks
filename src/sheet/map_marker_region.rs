use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown2: i16,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#unknown5: i16,
    pub r#unknown6: i16,
    pub r#unknown7: u16,
    pub r#unknown8: u16,
    pub r#unknown9: i16,
    pub r#unknown10: i16,
    pub r#unknown11: bool,
}
impl MapMarkerRegion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#x: row.field(1usize + offset)?.into_i16()?,
            r#unknown2: row.field(2usize + offset)?.into_i16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_i16()?,
            r#unknown6: row.field(6usize + offset)?.into_i16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_i16()?,
            r#unknown10: row.field(10usize + offset)?.into_i16()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
        })
    }
}
