use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
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
pub struct PlayerSearchLocation {}
impl PlayerSearchLocation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
