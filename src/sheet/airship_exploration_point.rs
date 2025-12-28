use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for AirshipExplorationPoint {
    fn name() -> String {
        "AirshipExplorationPoint".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AirshipExplorationPoint::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AirshipExplorationPoint {
    pub r#name: SeString,
    pub r#name_short: SeString,
    pub r#passengers: bool,
    pub r#x: i16,
    pub r#y: i16,
    pub r#rank_req: u8,
    pub r#ceruleum_tank_req: u16,
    pub r#survey_durationmin: u16,
    pub r#survey_distance: u16,
    pub r#unknown9: u8,
    pub r#surveillance_req: u8,
    pub r#unknown11: u8,
    pub r#unknown12: u8,
    pub r#exp_reward: u32,
}
impl AirshipExplorationPoint {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#name_short: row.field(1usize + offset)?.into_string()?,
            r#passengers: row.field(2usize + offset)?.into_bool()?,
            r#x: row.field(3usize + offset)?.into_i16()?,
            r#y: row.field(4usize + offset)?.into_i16()?,
            r#rank_req: row.field(5usize + offset)?.into_u8()?,
            r#ceruleum_tank_req: row.field(6usize + offset)?.into_u16()?,
            r#survey_durationmin: row.field(7usize + offset)?.into_u16()?,
            r#survey_distance: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#surveillance_req: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#exp_reward: row.field(13usize + offset)?.into_u32()?,
        })
    }
}
