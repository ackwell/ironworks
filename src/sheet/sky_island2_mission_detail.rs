use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for SkyIsland2MissionDetail {
    fn name() -> String {
        "SkyIsland2MissionDetail".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SkyIsland2MissionDetail::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SkyIsland2MissionDetail {
    pub r#type: u8,
    pub r#unknown1: u8,
    pub r#range: u8,
    pub r#unknown3: i8,
    pub r#e_obj: u32,
    pub r#unknown5: u32,
    pub r#unknown6: u32,
    pub r#objective: SeString,
}
impl SkyIsland2MissionDetail {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#range: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#e_obj: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#objective: row.field(7usize + offset)?.into_string()?,
        })
    }
}
