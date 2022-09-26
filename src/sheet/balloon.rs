use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Balloon {
    fn name() -> String {
        "Balloon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Balloon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Balloon {
    pub r#slowly: bool,
    pub r#dialogue: SeString,
}
impl Balloon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#slowly: row.field(0usize + offset)?.into_bool()?,
            r#dialogue: row.field(1usize + offset)?.into_string()?,
        })
    }
}
