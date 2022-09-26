use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FCAuthority {
    fn name() -> String {
        "FCAuthority".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCAuthority::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCAuthority {
    pub r#name: SeString,
    pub r#fc_authority_category: i32,
}
impl FCAuthority {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#fc_authority_category: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
