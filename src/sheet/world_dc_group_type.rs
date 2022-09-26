use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for WorldDCGroupType {
    fn name() -> String {
        "WorldDCGroupType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WorldDCGroupType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WorldDCGroupType {
    pub r#name: SeString,
    pub r#region: u8,
}
impl WorldDCGroupType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#region: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
