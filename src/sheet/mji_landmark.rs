use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
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
    pub r#unknown1: u16,
    pub r#sgb0: u16,
    pub r#unknown3: u8,
    pub r#sgb1: u16,
    pub r#unknown5: u8,
    pub r#sgb2: u16,
    pub r#unknown7: u8,
    pub r#sgb3: u16,
    pub r#unknown9: u8,
    pub r#sgb4: u16,
    pub r#unknown11: u8,
    pub r#unknown12: u8,
    pub r#unknown13: u8,
    pub r#unknown14: u8,
    pub r#unknown15: u8,
    pub r#unknown16: u8,
    pub r#material: Vec<u16>,
    pub r#amount: Vec<u8>,
    pub r#name: u32,
    pub r#unknown28: u16,
    pub r#icon: u32,
}
impl MJILandmark {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#sgb0: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#sgb1: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#sgb2: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#sgb3: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#sgb4: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#material: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(17usize + offset)?.into_u16()?) },
            )?,
            r#amount: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(22usize + offset)?.into_u8()?) },
            )?,
            r#name: row.field(27usize + offset)?.into_u32()?,
            r#unknown28: row.field(28usize + offset)?.into_u16()?,
            r#icon: row.field(29usize + offset)?.into_u32()?,
        })
    }
}
