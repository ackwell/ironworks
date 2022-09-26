use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for MountFlyingCondition {
    fn name() -> String {
        "MountFlyingCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MountFlyingCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MountFlyingCondition {
    pub r#quest: u32,
}
impl MountFlyingCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
