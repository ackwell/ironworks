use std::vec::Vec;
use ironworks::excel::Row;
use crate::utility::read_array;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for HousingUnitedExterior {
    fn name() -> String {
        "HousingUnitedExterior".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingUnitedExterior::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingUnitedExterior {
    pub r#unknown0: u8,
    pub r#item: Vec<u32>,
}
impl HousingUnitedExterior {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#item: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
