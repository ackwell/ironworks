use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for HouseRetainerPose {
    fn name() -> String {
        "HouseRetainerPose".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HouseRetainerPose::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HouseRetainerPose {
    pub r#action_timeline: u16,
}
impl HouseRetainerPose {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action_timeline: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
