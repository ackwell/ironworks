use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GFateRideShooting {
    fn name() -> String {
        "GFateRideShooting".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GFateRideShooting::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GFateRideShooting {
    pub r#content_entry: u32,
}
impl GFateRideShooting {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_entry: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
