use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for DpsChallenge {
    fn name() -> String {
        "DpsChallenge".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DpsChallenge::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DpsChallenge {
    pub r#player_level: u16,
    pub r#unknown1: bool,
    pub r#unknown2: u16,
    pub r#place_name: u16,
    pub r#icon: u32,
    pub r#order: u16,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl DpsChallenge {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#player_level: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#place_name: row.field(3usize + offset)?.into_u16()?,
            r#icon: row.field(4usize + offset)?.into_u32()?,
            r#order: row.field(5usize + offset)?.into_u16()?,
            r#name: row.field(6usize + offset)?.into_string()?,
            r#description: row.field(7usize + offset)?.into_string()?,
        })
    }
}
