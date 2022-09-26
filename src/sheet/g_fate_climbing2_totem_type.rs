use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GFateClimbing2TotemType {
    fn name() -> String {
        "GFateClimbing2TotemType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GFateClimbing2TotemType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GFateClimbing2TotemType {
    pub r#public_content_text_data: u32,
}
impl GFateClimbing2TotemType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#public_content_text_data: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
