use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown2: u16,
    pub r#unknown3: u8,
    pub r#unknown4: bool,
}
impl WorldDCGroupType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#region: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
