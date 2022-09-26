use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for GatheringPoint {
    fn name() -> String {
        "GatheringPoint".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringPoint::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringPoint {
    pub r#type: u8,
    pub r#unknown1: u8,
    pub r#gathering_point_base: i32,
    pub r#count: u8,
    pub r#gathering_point_bonus: Vec<u16>,
    pub r#territory_type: u16,
    pub r#place_name: u16,
    pub r#gathering_sub_category: u16,
}
impl GatheringPoint {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#gathering_point_base: row.field(2usize + offset)?.into_i32()?,
            r#count: row.field(3usize + offset)?.into_u8()?,
            r#gathering_point_bonus: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_u16()?) },
            )?,
            r#territory_type: row.field(6usize + offset)?.into_u16()?,
            r#place_name: row.field(7usize + offset)?.into_u16()?,
            r#gathering_sub_category: row.field(8usize + offset)?.into_u16()?,
        })
    }
}
