use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GFateClimbing2 {
    fn name() -> String {
        "GFateClimbing2".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GFateClimbing2::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GFateClimbing2 {
    pub r#content_entry: u32,
}
impl GFateClimbing2 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_entry: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
