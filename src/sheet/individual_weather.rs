use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for IndividualWeather {
    fn name() -> String {
        "IndividualWeather".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IndividualWeather::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IndividualWeather {
    pub r#weather: Vec<u8>,
    pub r#unknown6: u8,
    pub r#unknown7: u8,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
    pub r#unknown10: u8,
    pub r#unknown11: u8,
    pub r#unknown12: u8,
    pub r#unknown13: u8,
    pub r#unknown14: u32,
    pub r#quest: Vec<u32>,
    pub r#unknown21: u32,
    pub r#unknown22: u32,
    pub r#unknown23: u32,
    pub r#unknown24: u32,
    pub r#unknown25: u32,
    pub r#unknown26: u32,
    pub r#unknown27: u32,
}
impl IndividualWeather {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#weather: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#unknown14: row.field(14usize + offset)?.into_u32()?,
            r#quest: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(15usize + offset)?.into_u32()?) },
            )?,
            r#unknown21: row.field(21usize + offset)?.into_u32()?,
            r#unknown22: row.field(22usize + offset)?.into_u32()?,
            r#unknown23: row.field(23usize + offset)?.into_u32()?,
            r#unknown24: row.field(24usize + offset)?.into_u32()?,
            r#unknown25: row.field(25usize + offset)?.into_u32()?,
            r#unknown26: row.field(26usize + offset)?.into_u32()?,
            r#unknown27: row.field(27usize + offset)?.into_u32()?,
        })
    }
}
