use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for QuickChatTransient {
    fn name() -> String {
        "QuickChatTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuickChatTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuickChatTransient {
    pub r#text_output: SeString,
}
impl QuickChatTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text_output: row.field(0usize + offset)?.into_string()?,
        })
    }
}
