use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for BNpcState {
    fn name() -> String {
        "BNpcState".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BNpcState::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BNpcState {
    pub r#slot: u8,
    pub r#over_ray: i8,
    pub r#unknown2: i8,
    pub r#unknown3: u8,
    pub r#idle: u16,
    pub r#attribute0: u8,
    pub r#attribute_flag0: bool,
    pub r#attribute1: u8,
    pub r#attribute_flag1: bool,
    pub r#attribute2: u8,
    pub r#attribute_flag2: bool,
    pub r#scale: f32,
    pub r#unknown12: u8,
    pub r#loop_timeline: i32,
}
impl BNpcState {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#slot: row.field(0usize + offset)?.into_u8()?,
            r#over_ray: row.field(1usize + offset)?.into_i8()?,
            r#unknown2: row.field(2usize + offset)?.into_i8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#idle: row.field(4usize + offset)?.into_u16()?,
            r#attribute0: row.field(5usize + offset)?.into_u8()?,
            r#attribute_flag0: row.field(6usize + offset)?.into_bool()?,
            r#attribute1: row.field(7usize + offset)?.into_u8()?,
            r#attribute_flag1: row.field(8usize + offset)?.into_bool()?,
            r#attribute2: row.field(9usize + offset)?.into_u8()?,
            r#attribute_flag2: row.field(10usize + offset)?.into_bool()?,
            r#scale: row.field(11usize + offset)?.into_f32()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#loop_timeline: row.field(13usize + offset)?.into_i32()?,
        })
    }
}
