use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for SpearfishingNotebook {
    fn name() -> String {
        "SpearfishingNotebook".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SpearfishingNotebook::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SpearfishingNotebook {
    pub r#gathering_level: u8,
    pub r#is_shadow_node: bool,
    pub r#territory_type: i32,
    pub r#x: i16,
    pub r#y: i16,
    pub r#radius: u16,
    pub r#unknown6: u8,
    pub r#place_name: u16,
    pub r#unknown8: u8,
    pub r#gathering_point_base: u16,
    pub r#unknown10: u16,
    pub r#unknown11: u16,
}
impl SpearfishingNotebook {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_level: row.field(0usize + offset)?.into_u8()?,
            r#is_shadow_node: row.field(1usize + offset)?.into_bool()?,
            r#territory_type: row.field(2usize + offset)?.into_i32()?,
            r#x: row.field(3usize + offset)?.into_i16()?,
            r#y: row.field(4usize + offset)?.into_i16()?,
            r#radius: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#place_name: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#gathering_point_base: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u16()?,
        })
    }
}
