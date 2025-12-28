use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for SubmarineExploration {
    fn name() -> String {
        "SubmarineExploration".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SubmarineExploration::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SubmarineExploration {
    pub r#destination: SeString,
    pub r#location: SeString,
    pub r#x: i16,
    pub r#y: i16,
    pub r#z: i16,
    pub r#map: u8,
    pub r#starting_point: bool,
    pub r#stars: u8,
    pub r#rank_req: u8,
    pub r#ceruleum_tank_req: u8,
    pub r#survey_durationmin: u16,
    pub r#survey_distance: u8,
    pub r#exp_reward: u32,
}
impl SubmarineExploration {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#destination: row.field(0usize + offset)?.into_string()?,
            r#location: row.field(1usize + offset)?.into_string()?,
            r#x: row.field(2usize + offset)?.into_i16()?,
            r#y: row.field(3usize + offset)?.into_i16()?,
            r#z: row.field(4usize + offset)?.into_i16()?,
            r#map: row.field(5usize + offset)?.into_u8()?,
            r#starting_point: row.field(6usize + offset)?.into_bool()?,
            r#stars: row.field(7usize + offset)?.into_u8()?,
            r#rank_req: row.field(8usize + offset)?.into_u8()?,
            r#ceruleum_tank_req: row.field(9usize + offset)?.into_u8()?,
            r#survey_durationmin: row.field(10usize + offset)?.into_u16()?,
            r#survey_distance: row.field(11usize + offset)?.into_u8()?,
            r#exp_reward: row.field(12usize + offset)?.into_u32()?,
        })
    }
}
