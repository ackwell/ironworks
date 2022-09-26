use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for RetainerFortuneRewardRange {
    fn name() -> String {
        "RetainerFortuneRewardRange".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerFortuneRewardRange::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerFortuneRewardRange {
    pub r#percent_of_level: u16,
}
impl RetainerFortuneRewardRange {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#percent_of_level: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
