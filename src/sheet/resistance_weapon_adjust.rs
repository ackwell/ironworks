use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for ResistanceWeaponAdjust {
    fn name() -> String {
        "ResistanceWeaponAdjust".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ResistanceWeaponAdjust::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ResistanceWeaponAdjust {
    pub r#max_total_stats: u16,
    pub r#max_each_stat: u16,
    pub r#base_param: Vec<u8>,
    pub r#image: u32,
    pub r#unknown7: u8,
}
impl ResistanceWeaponAdjust {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#max_total_stats: row.field(0usize + offset)?.into_u16()?,
            r#max_each_stat: row.field(1usize + offset)?.into_u16()?,
            r#base_param: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u8()?) },
            )?,
            r#image: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
        })
    }
}
