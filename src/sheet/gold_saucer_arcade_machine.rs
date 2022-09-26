use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GoldSaucerArcadeMachine {
    fn name() -> String {
        "GoldSaucerArcadeMachine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GoldSaucerArcadeMachine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GoldSaucerArcadeMachine {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u16,
    pub r#unknown5: u8,
    pub r#fail_image: u32,
    pub r#unknown7: i8,
    pub r#unknown8: i8,
    pub r#unknown9: i8,
    pub r#unknown10: u32,
    pub r#unknown11: u8,
    pub r#unknown12: i8,
    pub r#unknown13: i8,
    pub r#unknown14: i8,
    pub r#unknown15: u8,
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
    pub r#unknown27: u32,
    pub r#unknown28: u32,
    pub r#unknown29: u32,
    pub r#unknown30: u32,
    pub r#unknown31: u8,
    pub r#unknown32: u8,
    pub r#unknown33: u8,
    pub r#unknown34: u8,
    pub r#poor: u32,
    pub r#good: u32,
    pub r#great: u32,
    pub r#excellent: u32,
}
impl GoldSaucerArcadeMachine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#fail_image: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#unknown8: row.field(8usize + offset)?.into_i8()?,
            r#unknown9: row.field(9usize + offset)?.into_i8()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_i8()?,
            r#unknown13: row.field(13usize + offset)?.into_i8()?,
            r#unknown14: row.field(14usize + offset)?.into_i8()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
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
            r#unknown27: row.field(27usize + offset)?.into_u32()?,
            r#unknown28: row.field(28usize + offset)?.into_u32()?,
            r#unknown29: row.field(29usize + offset)?.into_u32()?,
            r#unknown30: row.field(30usize + offset)?.into_u32()?,
            r#unknown31: row.field(31usize + offset)?.into_u8()?,
            r#unknown32: row.field(32usize + offset)?.into_u8()?,
            r#unknown33: row.field(33usize + offset)?.into_u8()?,
            r#unknown34: row.field(34usize + offset)?.into_u8()?,
            r#poor: row.field(35usize + offset)?.into_u32()?,
            r#good: row.field(36usize + offset)?.into_u32()?,
            r#great: row.field(37usize + offset)?.into_u32()?,
            r#excellent: row.field(38usize + offset)?.into_u32()?,
        })
    }
}
