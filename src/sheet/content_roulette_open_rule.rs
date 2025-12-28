use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ContentRouletteOpenRule {
    fn name() -> String {
        "ContentRouletteOpenRule".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentRouletteOpenRule::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentRouletteOpenRule {
    pub r#unknown0: bool,
    pub r#type: u32,
}
impl ContentRouletteOpenRule {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#type: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
