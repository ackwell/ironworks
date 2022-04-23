use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for HWDSharedGroupControlParam {
    fn name() -> String {
        "HWDSharedGroupControlParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDSharedGroupControlParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDSharedGroupControlParam {
    pub r#unknown0: u8,
    pub r#param_value: u8,
}
impl HWDSharedGroupControlParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#param_value: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
