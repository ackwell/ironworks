use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for GimmickAccessor {
    fn name() -> String {
        "GimmickAccessor".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GimmickAccessor::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GimmickAccessor {
    pub r#param0: i32,
    pub r#param1: u32,
    pub r#param2: u32,
    pub r#type: u32,
}
impl GimmickAccessor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#param0: row.field(0usize + offset)?.into_i32()?,
            r#param1: row.field(1usize + offset)?.into_u32()?,
            r#param2: row.field(2usize + offset)?.into_u32()?,
            r#type: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
