use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for AnimaWeaponFUITalk {
    fn name() -> String {
        "AnimaWeaponFUITalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeaponFUITalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeaponFUITalk {
    pub r#dialogue: i32,
}
impl AnimaWeaponFUITalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#dialogue: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
