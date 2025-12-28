use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for HousingAppeal {
    fn name() -> String {
        "HousingAppeal".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingAppeal::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingAppeal {
    pub r#tag: SeString,
    pub r#icon: u32,
    pub r#order: u8,
}
impl HousingAppeal {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#tag: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#order: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
