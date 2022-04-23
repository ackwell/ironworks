use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Stain {
    fn name() -> String {
        "Stain".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Stain::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Stain {
    pub r#color: u32,
    pub r#shade: u8,
    pub r#sub_order: u8,
    pub r#name: SeString,
}
impl Stain {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#color: row.field(0usize + offset)?.into_u32()?,
            r#shade: row.field(1usize + offset)?.into_u8()?,
            r#sub_order: row.field(2usize + offset)?.into_u8()?,
            r#name: row.field(3usize + offset)?.into_string()?,
        })
    }
}
