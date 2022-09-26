use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PlayerSearchSubLocation {
    fn name() -> String {
        "PlayerSearchSubLocation".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PlayerSearchSubLocation::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PlayerSearchSubLocation {}
impl PlayerSearchSubLocation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
