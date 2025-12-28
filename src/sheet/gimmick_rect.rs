use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GimmickRect {
    fn name() -> String {
        "GimmickRect".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GimmickRect::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GimmickRect {
    pub r#layout_id: u32,
    pub r#trigger_in: u8,
    pub r#param0: u32,
    pub r#unknown3: u32,
    pub r#unknown4: u32,
    pub r#unknown5: u32,
    pub r#trigger_out: u8,
    pub r#param1: u32,
    pub r#unknown8: u32,
    pub r#unknown9: u32,
    pub r#unknown10: u32,
}
impl GimmickRect {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#layout_id: row.field(0usize + offset)?.into_u32()?,
            r#trigger_in: row.field(1usize + offset)?.into_u8()?,
            r#param0: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#trigger_out: row.field(6usize + offset)?.into_u8()?,
            r#param1: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u32()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
        })
    }
}
