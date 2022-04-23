use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PublicContentCutscene {
    fn name() -> String {
        "PublicContentCutscene".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PublicContentCutscene::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PublicContentCutscene {
    pub r#cutscene: u32,
    pub r#cutscene2: u32,
}
impl PublicContentCutscene {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#cutscene: row.field(0usize + offset)?.into_u32()?,
            r#cutscene2: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
