use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FishParameter {
    fn name() -> String {
        "FishParameter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FishParameter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FishParameter {
    pub r#text: SeString,
    pub r#unknown1: SeString,
    pub r#unknown2: SeString,
    pub r#unknown3: SeString,
    pub r#item: i32,
    pub r#gathering_item_level: u16,
    pub r#ocean_stars: u8,
    pub r#unknown7: u16,
    pub r#is_hidden: bool,
    pub r#fishing_record_type: u8,
    pub r#fishing_spot: u16,
    pub r#gathering_sub_category: u16,
    pub r#is_in_log: bool,
    pub r#achievement_credit: u32,
}
impl FishParameter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_string()?,
            r#item: row.field(4usize + offset)?.into_i32()?,
            r#gathering_item_level: row.field(5usize + offset)?.into_u16()?,
            r#ocean_stars: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#is_hidden: row.field(8usize + offset)?.into_bool()?,
            r#fishing_record_type: row.field(9usize + offset)?.into_u8()?,
            r#fishing_spot: row.field(10usize + offset)?.into_u16()?,
            r#gathering_sub_category: row.field(11usize + offset)?.into_u16()?,
            r#is_in_log: row.field(12usize + offset)?.into_bool()?,
            r#achievement_credit: row.field(13usize + offset)?.into_u32()?,
        })
    }
}
