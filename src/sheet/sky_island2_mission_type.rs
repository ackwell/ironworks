use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SkyIsland2MissionType {
    fn name() -> String {
        "SkyIsland2MissionType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SkyIsland2MissionType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SkyIsland2MissionType {
    pub r#type: bool,
}
impl SkyIsland2MissionType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_bool()?,
        })
    }
}
