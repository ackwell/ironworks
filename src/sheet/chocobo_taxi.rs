use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for ChocoboTaxi {
    fn name() -> String {
        "ChocoboTaxi".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboTaxi::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboTaxi {
    pub r#location: u32,
    pub r#fare: u8,
    pub r#time_required: u16,
}
impl ChocoboTaxi {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_u32()?,
            r#fare: row.field(1usize + offset)?.into_u8()?,
            r#time_required: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
