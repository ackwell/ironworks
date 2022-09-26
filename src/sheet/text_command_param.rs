use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for TextCommandParam {
    fn name() -> String {
        "TextCommandParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TextCommandParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TextCommandParam {
    pub r#param: SeString,
}
impl TextCommandParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#param: row.field(0usize + offset)?.into_string()?,
        })
    }
}
