use ironworks::sestring::SeString;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use crate::utility::read_array;
use ironworks::excel::Row;
impl MetadataAdapter for ChocoboTaxiStand {
    fn name() -> String {
        "ChocoboTaxiStand".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboTaxiStand::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboTaxiStand {
    pub r#target_locations: Vec<u16>,
    pub r#place_name: SeString,
}
impl ChocoboTaxiStand {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#target_locations: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#place_name: row.field(8usize + offset)?.into_string()?,
        })
    }
}
