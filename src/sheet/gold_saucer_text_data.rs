use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for GoldSaucerTextData {
    fn name() -> String {
        "GoldSaucerTextData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GoldSaucerTextData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GoldSaucerTextData {
    pub r#text: SeString,
}
impl GoldSaucerTextData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
