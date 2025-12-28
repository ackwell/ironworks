use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for ChocoboRaceAbility {
    fn name() -> String {
        "ChocoboRaceAbility".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceAbility::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceAbility {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: u32,
    pub r#chocobo_race_ability_type: i8,
    pub r#value: u8,
}
impl ChocoboRaceAbility {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u32()?,
            r#chocobo_race_ability_type: row.field(3usize + offset)?.into_i8()?,
            r#value: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
