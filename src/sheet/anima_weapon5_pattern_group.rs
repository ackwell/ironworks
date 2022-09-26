use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for AnimaWeapon5PatternGroup {
    fn name() -> String {
        "AnimaWeapon5PatternGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeapon5PatternGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5PatternGroup {
    pub r#name: SeString,
}
impl AnimaWeapon5PatternGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
