use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SkyIsland2Mission {
    fn name() -> String {
        "SkyIsland2Mission".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SkyIsland2Mission::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SkyIsland2Mission {
    pub r#item1: u32,
    pub r#item2: u32,
    pub r#place_name: u16,
    pub r#unknown3: u16,
    pub r#objective1: u16,
    pub r#pop_range0: u32,
    pub r#required_amount1: u8,
    pub r#unknown7: u32,
    pub r#unknown8: u8,
    pub r#objective2: u16,
    pub r#pop_range1: u32,
    pub r#required_amount2: u8,
    pub r#unknown12: u32,
    pub r#unknown13: u8,
    pub r#objective3: u16,
    pub r#pop_range2: u32,
    pub r#unknown16: u8,
    pub r#unknown17: u32,
    pub r#unknown18: u8,
    pub r#unknown19: u32,
    pub r#image: u32,
}
impl SkyIsland2Mission {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item1: row.field(0usize + offset)?.into_u32()?,
            r#item2: row.field(1usize + offset)?.into_u32()?,
            r#place_name: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#objective1: row.field(4usize + offset)?.into_u16()?,
            r#pop_range0: row.field(5usize + offset)?.into_u32()?,
            r#required_amount1: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#objective2: row.field(9usize + offset)?.into_u16()?,
            r#pop_range1: row.field(10usize + offset)?.into_u32()?,
            r#required_amount2: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u32()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#objective3: row.field(14usize + offset)?.into_u16()?,
            r#pop_range2: row.field(15usize + offset)?.into_u32()?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#unknown17: row.field(17usize + offset)?.into_u32()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u32()?,
            r#image: row.field(20usize + offset)?.into_u32()?,
        })
    }
}
