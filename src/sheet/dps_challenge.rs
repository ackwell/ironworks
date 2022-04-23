use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
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
    pub r#place_name: bool,
    pub r#icon: u16,
    pub r#order: u16,
    pub r#name: u32,
    pub r#description: u16,
}
impl DpsChallenge {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#player_level: row.field(0usize + offset)?.into_u16()?,
            r#place_name: row.field(1usize + offset)?.into_bool()?,
            r#icon: row.field(2usize + offset)?.into_u16()?,
            r#order: row.field(3usize + offset)?.into_u16()?,
            r#name: row.field(4usize + offset)?.into_u32()?,
            r#description: row.field(5usize + offset)?.into_u16()?,
        })
    }
}
