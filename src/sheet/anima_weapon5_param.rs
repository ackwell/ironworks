use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for AnimaWeapon5Param {
    fn name() -> String {
        "AnimaWeapon5Param".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeapon5Param::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5Param {
    pub r#base_param: u8,
    pub r#name: SeString,
}
impl AnimaWeapon5Param {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base_param: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
