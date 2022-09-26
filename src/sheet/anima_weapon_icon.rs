use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AnimaWeaponIcon {
    fn name() -> String {
        "AnimaWeaponIcon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeaponIcon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeaponIcon {
    pub r#hyperconductive: i32,
    pub r#reborn: i32,
    pub r#sharpened: i32,
    pub r#zodiac: i32,
    pub r#zodiac_lux: i32,
}
impl AnimaWeaponIcon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#hyperconductive: row.field(0usize + offset)?.into_i32()?,
            r#reborn: row.field(1usize + offset)?.into_i32()?,
            r#sharpened: row.field(2usize + offset)?.into_i32()?,
            r#zodiac: row.field(3usize + offset)?.into_i32()?,
            r#zodiac_lux: row.field(4usize + offset)?.into_i32()?,
        })
    }
}
