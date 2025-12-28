use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for QuestRedo {
    fn name() -> String {
        "QuestRedo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRedo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRedo {
    pub r#final_quest: u32,
    pub r#unknown1: u32,
    pub r#unknown2: u8,
    pub r#chapter: u16,
    pub r#quest: Vec<u32>,
    pub r#unknown36: u8,
    pub r#unknown37: u8,
    pub r#unknown38: u8,
    pub r#unknown39: u8,
    pub r#unknown40: u8,
    pub r#unknown41: u8,
    pub r#unknown42: u8,
    pub r#unknown43: u8,
    pub r#unknown44: u8,
    pub r#unknown45: u8,
    pub r#unknown46: u8,
    pub r#unknown47: u8,
    pub r#unknown48: u8,
    pub r#unknown49: u8,
    pub r#unknown50: u8,
    pub r#unknown51: u8,
    pub r#unknown52: u8,
    pub r#unknown53: u8,
    pub r#unknown54: u8,
    pub r#unknown55: u8,
    pub r#unknown56: u8,
    pub r#unknown57: u8,
    pub r#unknown58: u8,
    pub r#unknown59: u8,
    pub r#unknown60: u8,
    pub r#unknown61: u8,
    pub r#unknown62: u8,
    pub r#unknown63: u8,
    pub r#unknown64: u8,
    pub r#unknown65: u8,
    pub r#unknown66: u8,
    pub r#unknown67: u8,
}
impl QuestRedo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#final_quest: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#chapter: row.field(3usize + offset)?.into_u16()?,
            r#quest: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_u32()?) },
            )?,
            r#unknown36: row.field(36usize + offset)?.into_u8()?,
            r#unknown37: row.field(37usize + offset)?.into_u8()?,
            r#unknown38: row.field(38usize + offset)?.into_u8()?,
            r#unknown39: row.field(39usize + offset)?.into_u8()?,
            r#unknown40: row.field(40usize + offset)?.into_u8()?,
            r#unknown41: row.field(41usize + offset)?.into_u8()?,
            r#unknown42: row.field(42usize + offset)?.into_u8()?,
            r#unknown43: row.field(43usize + offset)?.into_u8()?,
            r#unknown44: row.field(44usize + offset)?.into_u8()?,
            r#unknown45: row.field(45usize + offset)?.into_u8()?,
            r#unknown46: row.field(46usize + offset)?.into_u8()?,
            r#unknown47: row.field(47usize + offset)?.into_u8()?,
            r#unknown48: row.field(48usize + offset)?.into_u8()?,
            r#unknown49: row.field(49usize + offset)?.into_u8()?,
            r#unknown50: row.field(50usize + offset)?.into_u8()?,
            r#unknown51: row.field(51usize + offset)?.into_u8()?,
            r#unknown52: row.field(52usize + offset)?.into_u8()?,
            r#unknown53: row.field(53usize + offset)?.into_u8()?,
            r#unknown54: row.field(54usize + offset)?.into_u8()?,
            r#unknown55: row.field(55usize + offset)?.into_u8()?,
            r#unknown56: row.field(56usize + offset)?.into_u8()?,
            r#unknown57: row.field(57usize + offset)?.into_u8()?,
            r#unknown58: row.field(58usize + offset)?.into_u8()?,
            r#unknown59: row.field(59usize + offset)?.into_u8()?,
            r#unknown60: row.field(60usize + offset)?.into_u8()?,
            r#unknown61: row.field(61usize + offset)?.into_u8()?,
            r#unknown62: row.field(62usize + offset)?.into_u8()?,
            r#unknown63: row.field(63usize + offset)?.into_u8()?,
            r#unknown64: row.field(64usize + offset)?.into_u8()?,
            r#unknown65: row.field(65usize + offset)?.into_u8()?,
            r#unknown66: row.field(66usize + offset)?.into_u8()?,
            r#unknown67: row.field(67usize + offset)?.into_u8()?,
        })
    }
}
