use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for GameRewardObtainType {
    fn name() -> String {
        "GameRewardObtainType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GameRewardObtainType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GameRewardObtainType {}
impl GameRewardObtainType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
