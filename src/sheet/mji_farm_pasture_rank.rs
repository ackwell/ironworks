use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MJIFarmPastureRank {
    fn name() -> String {
        "MJIFarmPastureRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIFarmPastureRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIFarmPastureRank_SGB {
    pub r#sgb: Vec<u32>,
}
impl MJIFarmPastureRank_SGB {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct MJIFarmPastureRank {
    pub r#sgb: Vec<MJIFarmPastureRank_SGB>,
    pub r#unknown16: u8,
    pub r#unknown17: u8,
    pub r#unknown18: u8,
    pub r#unknown19: u8,
    pub r#unknown20: u8,
    pub r#unknown21: u8,
    pub r#unknown22: u8,
    pub r#unknown23: u8,
    pub r#unknown24: u8,
    pub r#unknown25: u8,
    pub r#unknown26: u8,
    pub r#unknown27: u8,
    pub r#unknown28: u32,
    pub r#unknown29: u32,
    pub r#unknown30: u32,
    pub r#unknown31: u32,
    pub r#unknown32: u32,
    pub r#unknown33: u32,
    pub r#unknown34: u32,
    pub r#unknown35: u32,
    pub r#unknown36: u8,
    pub r#unknown37: u8,
    pub r#unknown38: u8,
    pub r#unknown39: u8,
    pub r#unknown40: u16,
    pub r#unknown41: u16,
    pub r#unknown42: u16,
    pub r#unknown43: u16,
    pub r#unknown44: u16,
    pub r#unknown45: u16,
    pub r#unknown46: u16,
    pub r#unknown47: u16,
}
impl MJIFarmPastureRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: read_array(
                offset,
                4usize,
                4usize,
                |offset| { Result::Ok(MJIFarmPastureRank_SGB::populate(row, offset)?) },
            )?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u8()?,
            r#unknown20: row.field(20usize + offset)?.into_u8()?,
            r#unknown21: row.field(21usize + offset)?.into_u8()?,
            r#unknown22: row.field(22usize + offset)?.into_u8()?,
            r#unknown23: row.field(23usize + offset)?.into_u8()?,
            r#unknown24: row.field(24usize + offset)?.into_u8()?,
            r#unknown25: row.field(25usize + offset)?.into_u8()?,
            r#unknown26: row.field(26usize + offset)?.into_u8()?,
            r#unknown27: row.field(27usize + offset)?.into_u8()?,
            r#unknown28: row.field(28usize + offset)?.into_u32()?,
            r#unknown29: row.field(29usize + offset)?.into_u32()?,
            r#unknown30: row.field(30usize + offset)?.into_u32()?,
            r#unknown31: row.field(31usize + offset)?.into_u32()?,
            r#unknown32: row.field(32usize + offset)?.into_u32()?,
            r#unknown33: row.field(33usize + offset)?.into_u32()?,
            r#unknown34: row.field(34usize + offset)?.into_u32()?,
            r#unknown35: row.field(35usize + offset)?.into_u32()?,
            r#unknown36: row.field(36usize + offset)?.into_u8()?,
            r#unknown37: row.field(37usize + offset)?.into_u8()?,
            r#unknown38: row.field(38usize + offset)?.into_u8()?,
            r#unknown39: row.field(39usize + offset)?.into_u8()?,
            r#unknown40: row.field(40usize + offset)?.into_u16()?,
            r#unknown41: row.field(41usize + offset)?.into_u16()?,
            r#unknown42: row.field(42usize + offset)?.into_u16()?,
            r#unknown43: row.field(43usize + offset)?.into_u16()?,
            r#unknown44: row.field(44usize + offset)?.into_u16()?,
            r#unknown45: row.field(45usize + offset)?.into_u16()?,
            r#unknown46: row.field(46usize + offset)?.into_u16()?,
            r#unknown47: row.field(47usize + offset)?.into_u16()?,
        })
    }
}
