use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for HWDInfoBoardArticleType {
    fn name() -> String {
        "HWDInfoBoardArticleType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDInfoBoardArticleType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDInfoBoardArticleType {
    pub r#type: SeString,
}
impl HWDInfoBoardArticleType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_string()?,
        })
    }
}
