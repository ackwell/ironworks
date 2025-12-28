use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for PlayerSearchLocation {
    fn name() -> String {
        "PlayerSearchLocation".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PlayerSearchLocation::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PlayerSearchLocation {
    pub r#unknown0: u8,
    pub r#unknown1: SeString,
}
impl PlayerSearchLocation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
        })
    }
}
