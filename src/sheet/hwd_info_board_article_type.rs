use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
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
