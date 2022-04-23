use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for HWDGathererInspectionReward {
    fn name() -> String {
        "HWDGathererInspectionReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDGathererInspectionReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDGathererInspectionReward {
    pub r#scrips: u16,
    pub r#points: u16,
}
impl HWDGathererInspectionReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#scrips: row.field(0usize + offset)?.into_u16()?,
            r#points: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
