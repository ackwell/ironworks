use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MJILandmark {
    fn name() -> String {
        "MJILandmark".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJILandmark::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJILandmark {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u16,
    pub r#sgb0: u16,
    pub r#sgb1: u16,
    pub r#sgb2: u16,
    pub r#unknown6: u8,
    pub r#sgb3: u16,
    pub r#unknown8: u8,
    pub r#sgb4: u16,
    pub r#unknown10: u8,
    pub r#sgb5: u16,
    pub r#unknown12: u8,
    pub r#sgb6: u16,
    pub r#unknown14: u8,
    pub r#unknown15: u8,
    pub r#unknown16: u8,
    pub r#unknown17: u8,
    pub r#unknown18: u8,
    pub r#unknown19: u8,
    pub r#material: Vec<u16>,
    pub r#amount: Vec<u8>,
    pub r#name: u32,
    pub r#unknown31: u16,
    pub r#icon: u32,
}
impl MJILandmark {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#sgb0: row.field(3usize + offset)?.into_u16()?,
            r#sgb1: row.field(4usize + offset)?.into_u16()?,
            r#sgb2: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#sgb3: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#sgb4: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#sgb5: row.field(11usize + offset)?.into_u16()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#sgb6: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u8()?,
            r#material: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(20usize + offset)?.into_u16()?) },
            )?,
            r#amount: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(25usize + offset)?.into_u8()?) },
            )?,
            r#name: row.field(30usize + offset)?.into_u32()?,
            r#unknown31: row.field(31usize + offset)?.into_u16()?,
            r#icon: row.field(32usize + offset)?.into_u32()?,
        })
    }
}
