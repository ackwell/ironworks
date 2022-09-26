use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for Behavior {
    fn name() -> String {
        "Behavior".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Behavior::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Behavior {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#condition0_target: u8,
    pub r#condition0_type: u8,
    pub r#balloon: i32,
    pub r#unknown5: i16,
    pub r#unknown6: i32,
    pub r#unknown7: u16,
    pub r#unknown8: u16,
    pub r#condition1_target: u8,
    pub r#condition1_type: u8,
    pub r#content_argument0: u32,
    pub r#content_argument1: u8,
}
impl Behavior {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#condition0_target: row.field(2usize + offset)?.into_u8()?,
            r#condition0_type: row.field(3usize + offset)?.into_u8()?,
            r#balloon: row.field(4usize + offset)?.into_i32()?,
            r#unknown5: row.field(5usize + offset)?.into_i16()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#condition1_target: row.field(9usize + offset)?.into_u8()?,
            r#condition1_type: row.field(10usize + offset)?.into_u8()?,
            r#content_argument0: row.field(11usize + offset)?.into_u32()?,
            r#content_argument1: row.field(12usize + offset)?.into_u8()?,
        })
    }
}
