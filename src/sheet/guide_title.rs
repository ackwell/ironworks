use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GuideTitle {
    fn name() -> String {
        "GuideTitle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuideTitle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuideTitle {
    pub r#title: SeString,
}
impl GuideTitle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#title: row.field(0usize + offset)?.into_string()?,
        })
    }
}
