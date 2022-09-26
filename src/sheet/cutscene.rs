use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for Cutscene {
    fn name() -> String {
        "Cutscene".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Cutscene::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Cutscene {
    pub r#path: SeString,
}
impl Cutscene {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#path: row.field(0usize + offset)?.into_string()?,
        })
    }
}
