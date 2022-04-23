use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PlaceName {
    fn name() -> String {
        "PlaceName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PlaceName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PlaceName {
    pub r#name: SeString,
    pub r#unknown1: i8,
    pub r#name_no_article: SeString,
}
impl PlaceName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#name_no_article: row.field(2usize + offset)?.into_string()?,
        })
    }
}
