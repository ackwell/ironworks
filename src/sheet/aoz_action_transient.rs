use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for AozActionTransient {
    fn name() -> String {
        "AozActionTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AozActionTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AozActionTransient {
    pub r#number: u8,
    pub r#icon: u32,
    pub r#stats: SeString,
    pub r#description: SeString,
    pub r#location_key: u8,
    pub r#location: u16,
    pub r#required_for_quest: u32,
    pub r#previous_quest: u32,
    pub r#targets_enemy: bool,
    pub r#targets_self_or_ally: bool,
    pub r#cause_slow: bool,
    pub r#cause_petrify: bool,
    pub r#cause_paralysis: bool,
    pub r#cause_interrupt: bool,
    pub r#cause_blind: bool,
    pub r#cause_stun: bool,
    pub r#cause_sleep: bool,
    pub r#cause_bind: bool,
    pub r#cause_heavy: bool,
    pub r#cause_death: bool,
}
impl AozActionTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#number: row.field(0usize + offset)?.into_u8()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#stats: row.field(2usize + offset)?.into_string()?,
            r#description: row.field(3usize + offset)?.into_string()?,
            r#location_key: row.field(4usize + offset)?.into_u8()?,
            r#location: row.field(5usize + offset)?.into_u16()?,
            r#required_for_quest: row.field(6usize + offset)?.into_u32()?,
            r#previous_quest: row.field(7usize + offset)?.into_u32()?,
            r#targets_enemy: row.field(8usize + offset)?.into_bool()?,
            r#targets_self_or_ally: row.field(9usize + offset)?.into_bool()?,
            r#cause_slow: row.field(10usize + offset)?.into_bool()?,
            r#cause_petrify: row.field(11usize + offset)?.into_bool()?,
            r#cause_paralysis: row.field(12usize + offset)?.into_bool()?,
            r#cause_interrupt: row.field(13usize + offset)?.into_bool()?,
            r#cause_blind: row.field(14usize + offset)?.into_bool()?,
            r#cause_stun: row.field(15usize + offset)?.into_bool()?,
            r#cause_sleep: row.field(16usize + offset)?.into_bool()?,
            r#cause_bind: row.field(17usize + offset)?.into_bool()?,
            r#cause_heavy: row.field(18usize + offset)?.into_bool()?,
            r#cause_death: row.field(19usize + offset)?.into_bool()?,
        })
    }
}
