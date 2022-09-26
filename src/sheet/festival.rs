use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for Festival {
    fn name() -> String {
        "Festival".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Festival::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Festival {
    pub r#name: SeString,
}
impl Festival {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
