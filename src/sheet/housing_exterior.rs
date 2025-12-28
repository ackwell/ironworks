use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for HousingExterior {
    fn name() -> String {
        "HousingExterior".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingExterior::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingExterior {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#place_name: u16,
    pub r#housing_size: u8,
    pub r#model: SeString,
}
impl HousingExterior {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#place_name: row.field(2usize + offset)?.into_u16()?,
            r#housing_size: row.field(3usize + offset)?.into_u8()?,
            r#model: row.field(4usize + offset)?.into_string()?,
        })
    }
}
