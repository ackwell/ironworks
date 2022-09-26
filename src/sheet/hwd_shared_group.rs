use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for HWDSharedGroup {
    fn name() -> String {
        "HWDSharedGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDSharedGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDSharedGroup {
    pub r#lgb_shared_group: u32,
    pub r#param: u8,
}
impl HWDSharedGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#lgb_shared_group: row.field(0usize + offset)?.into_u32()?,
            r#param: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
