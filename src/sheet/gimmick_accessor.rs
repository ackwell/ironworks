use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
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
    pub r#unknown4: u32,
    pub r#unknown5: u32,
    pub r#unknown6: u32,
    pub r#unknown7: bool,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: bool,
}
impl GimmickAccessor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#param0: row.field(0usize + offset)?.into_i32()?,
            r#param1: row.field(1usize + offset)?.into_u32()?,
            r#param2: row.field(2usize + offset)?.into_u32()?,
            r#type: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
        })
    }
}
