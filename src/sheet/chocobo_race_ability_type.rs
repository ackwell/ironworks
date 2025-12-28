use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ChocoboRaceAbilityType {
    fn name() -> String {
        "ChocoboRaceAbilityType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceAbilityType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceAbilityType {
    pub r#is_active: bool,
}
impl ChocoboRaceAbilityType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#is_active: row.field(0usize + offset)?.into_bool()?,
        })
    }
}
