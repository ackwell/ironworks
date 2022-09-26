use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for AnimaWeapon5SpiritTalk {
    fn name() -> String {
        "AnimaWeapon5SpiritTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeapon5SpiritTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5SpiritTalk {
    pub r#dialogue: i32,
}
impl AnimaWeapon5SpiritTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#dialogue: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
