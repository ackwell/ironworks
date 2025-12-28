use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GathererReductionReward {
    fn name() -> String {
        "GathererReductionReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GathererReductionReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GathererReductionReward {
    pub r#unknown0: u16,
    pub r#unknown1: bool,
}
impl GathererReductionReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
        })
    }
}
