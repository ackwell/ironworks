use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Channeling {
    fn name() -> String {
        "Channeling".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Channeling::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Channeling {
    pub r#file: SeString,
    pub r#width_scale: u8,
}
impl Channeling {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#file: row.field(0usize + offset)?.into_string()?,
            r#width_scale: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
