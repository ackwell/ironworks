use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::utility::read_array;
use crate::error::PopulateError;
use std::vec::Vec;
use ironworks::excel::Row;
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
