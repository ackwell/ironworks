use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DawnContent {
    fn name() -> String {
        "DawnContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnContent {
    pub r#content: u32,
    pub r#unknown1: bool,
    pub r#unknown2: u16,
    pub r#unknown3: bool,
    pub r#exp_below_ex_max_lvl: bool,
    pub r#exp_above_ex_max_lvl: bool,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#unknown8: u32,
    pub r#unknown9: u8,
    pub r#unknown10: u32,
    pub r#unknown11: u32,
    pub r#unknown12: u32,
    pub r#unknown13: u32,
    pub r#unknown14: u32,
    pub r#unknown15: u32,
    pub r#unknown16: u32,
    pub r#unknown17: u32,
    pub r#unknown18: u32,
    pub r#unknown19: u32,
}
impl DawnContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#exp_below_ex_max_lvl: row.field(4usize + offset)?.into_bool()?,
            r#exp_above_ex_max_lvl: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u32()?,
            r#unknown12: row.field(12usize + offset)?.into_u32()?,
            r#unknown13: row.field(13usize + offset)?.into_u32()?,
            r#unknown14: row.field(14usize + offset)?.into_u32()?,
            r#unknown15: row.field(15usize + offset)?.into_u32()?,
            r#unknown16: row.field(16usize + offset)?.into_u32()?,
            r#unknown17: row.field(17usize + offset)?.into_u32()?,
            r#unknown18: row.field(18usize + offset)?.into_u32()?,
            r#unknown19: row.field(19usize + offset)?.into_u32()?,
        })
    }
}
