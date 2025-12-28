use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for GcArmyMemberGrow {
    fn name() -> String {
        "GcArmyMemberGrow".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyMemberGrow::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyMemberGrow {
    pub r#class_job: u8,
    pub r#class_book: i32,
    pub r#equip_preset: Vec<u16>,
    pub r#unknown62: u16,
    pub r#physical: Vec<u8>,
    pub r#unknown123: u8,
    pub r#mental: Vec<u8>,
    pub r#unknown184: u8,
    pub r#tactical: Vec<u8>,
    pub r#unknown245: u8,
}
impl GcArmyMemberGrow {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#class_job: row.field(0usize + offset)?.into_u8()?,
            r#class_book: row.field(1usize + offset)?.into_i32()?,
            r#equip_preset: read_array(
                offset,
                60usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u16()?) },
            )?,
            r#unknown62: row.field(62usize + offset)?.into_u16()?,
            r#physical: read_array(
                offset,
                60usize,
                1usize,
                |offset| { Result::Ok(row.field(63usize + offset)?.into_u8()?) },
            )?,
            r#unknown123: row.field(123usize + offset)?.into_u8()?,
            r#mental: read_array(
                offset,
                60usize,
                1usize,
                |offset| { Result::Ok(row.field(124usize + offset)?.into_u8()?) },
            )?,
            r#unknown184: row.field(184usize + offset)?.into_u8()?,
            r#tactical: read_array(
                offset,
                60usize,
                1usize,
                |offset| { Result::Ok(row.field(185usize + offset)?.into_u8()?) },
            )?,
            r#unknown245: row.field(245usize + offset)?.into_u8()?,
        })
    }
}
