use std::vec::Vec;
use crate::utility::read_array;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for HousingEmploymentNpcList {
    fn name() -> String {
        "HousingEmploymentNpcList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingEmploymentNpcList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingEmploymentNpcList {
    pub r#race: u8,
    pub r#e_npc_base: Vec<u32>,
}
impl HousingEmploymentNpcList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#race: row.field(0usize + offset)?.into_u8()?,
            r#e_npc_base: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
