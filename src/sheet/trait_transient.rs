use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for TraitTransient {
    fn name() -> String {
        "TraitTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TraitTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TraitTransient {
    pub r#description: SeString,
}
impl TraitTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
        })
    }
}
