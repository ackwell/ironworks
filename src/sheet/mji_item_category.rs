use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MJIItemCategory {
    fn name() -> String {
        "MJIItemCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIItemCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIItemCategory {
    pub r#singular: SeString,
    pub r#plural: SeString,
}
impl MJIItemCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#singular: row.field(0usize + offset)?.into_string()?,
            r#plural: row.field(1usize + offset)?.into_string()?,
        })
    }
}
