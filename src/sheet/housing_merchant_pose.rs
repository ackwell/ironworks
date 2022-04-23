use ironworks::sestring::SeString;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for HousingMerchantPose {
    fn name() -> String {
        "HousingMerchantPose".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingMerchantPose::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingMerchantPose {
    pub r#action_timeline: u16,
    pub r#pose: SeString,
}
impl HousingMerchantPose {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action_timeline: row.field(0usize + offset)?.into_u16()?,
            r#pose: row.field(1usize + offset)?.into_string()?,
        })
    }
}
