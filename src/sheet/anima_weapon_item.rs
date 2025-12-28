use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for AnimaWeaponItem {
    fn name() -> String {
        "AnimaWeaponItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeaponItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeaponItem {
    pub r#item: Vec<u32>,
}
impl AnimaWeaponItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: read_array(
                offset,
                14usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
