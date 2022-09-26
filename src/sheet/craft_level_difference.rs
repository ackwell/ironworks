use std::convert::Infallible;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for CraftLevelDifference {
    fn name() -> String {
        "CraftLevelDifference".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CraftLevelDifference::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CraftLevelDifference {
    pub r#difference: i16,
    pub r#progress_factor: Option<Infallible>,
    pub r#quality_factor: Option<Infallible>,
}
impl CraftLevelDifference {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#difference: row.field(0usize + offset)?.into_i16()?,
            r#progress_factor: None,
            r#quality_factor: None,
        })
    }
}
