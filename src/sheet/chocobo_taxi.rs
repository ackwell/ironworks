use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown3: u16,
    pub r#unknown4: bool,
}
impl ChocoboTaxi {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_u32()?,
            r#fare: row.field(1usize + offset)?.into_u8()?,
            r#time_required: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
