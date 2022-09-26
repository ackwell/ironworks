use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::utility::read_array;
use std::vec::Vec;
impl MetadataAdapter for AnimaWeapon5 {
    fn name() -> String {
        "AnimaWeapon5".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeapon5::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5 {
    pub r#item: i32,
    pub r#unknown1: u8,
    pub r#secondary_stat_total: u8,
    pub r#parameter: Vec<u8>,
}
impl AnimaWeapon5 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#secondary_stat_total: row.field(2usize + offset)?.into_u8()?,
            r#parameter: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
