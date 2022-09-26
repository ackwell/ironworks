use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GFateClimbing2Content {
    fn name() -> String {
        "GFateClimbing2Content".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GFateClimbing2Content::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GFateClimbing2Content {
    pub r#public_content_text_data: u32,
}
impl GFateClimbing2Content {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#public_content_text_data: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
