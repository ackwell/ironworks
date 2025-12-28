use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Tutorial {
    fn name() -> String {
        "Tutorial".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Tutorial::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Tutorial {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#exp: u8,
    pub r#gil: u32,
    pub r#reward_tank: u32,
    pub r#reward_melee: u32,
    pub r#reward_ranged: u32,
    pub r#objective: u32,
    pub r#unknown10: u32,
    pub r#unknown11: u32,
}
impl Tutorial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#exp: row.field(4usize + offset)?.into_u8()?,
            r#gil: row.field(5usize + offset)?.into_u32()?,
            r#reward_tank: row.field(6usize + offset)?.into_u32()?,
            r#reward_melee: row.field(7usize + offset)?.into_u32()?,
            r#reward_ranged: row.field(8usize + offset)?.into_u32()?,
            r#objective: row.field(9usize + offset)?.into_u32()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u32()?,
        })
    }
}
