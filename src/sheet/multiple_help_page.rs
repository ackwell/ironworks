use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MultipleHelpPage {
    fn name() -> String {
        "MultipleHelpPage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MultipleHelpPage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MultipleHelpPage {
    pub r#unknown0: u16,
}
impl MultipleHelpPage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
