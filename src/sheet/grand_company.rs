use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
impl MetadataAdapter for GrandCompany {
    fn name() -> String {
        "GrandCompany".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GrandCompany::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GrandCompany {
    pub r#name: SeString,
}
impl GrandCompany {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
