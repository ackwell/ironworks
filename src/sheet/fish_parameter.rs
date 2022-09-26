use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
    pub r#item: i32,
    pub r#gathering_item_level: u16,
    pub r#unknown3: u8,
    pub r#is_hidden: bool,
    pub r#unknown5: bool,
    pub r#fishing_record_type: u8,
    pub r#territory_type: i32,
    pub r#gathering_sub_category: u16,
    pub r#is_in_log: bool,
    pub r#time_restricted: bool,
    pub r#weather_restricted: bool,
}
impl FishParameter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#item: row.field(1usize + offset)?.into_i32()?,
            r#gathering_item_level: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#is_hidden: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#fishing_record_type: row.field(6usize + offset)?.into_u8()?,
            r#territory_type: row.field(7usize + offset)?.into_i32()?,
            r#gathering_sub_category: row.field(8usize + offset)?.into_u16()?,
            r#is_in_log: row.field(9usize + offset)?.into_bool()?,
            r#time_restricted: row.field(10usize + offset)?.into_bool()?,
            r#weather_restricted: row.field(11usize + offset)?.into_bool()?,
        })
    }
}
