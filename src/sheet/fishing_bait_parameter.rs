use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for FishingBaitParameter {
    fn name() -> String {
        "FishingBaitParameter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FishingBaitParameter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FishingBaitParameter {
    pub r#unknown0: u32,
}
impl FishingBaitParameter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
