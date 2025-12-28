use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for HWDInfoBoardArticleTransient {
    fn name() -> String {
        "HWDInfoBoardArticleTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDInfoBoardArticleTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDInfoBoardArticleTransient {
    pub r#image: u32,
    pub r#text: SeString,
    pub r#npc_name: SeString,
}
impl HWDInfoBoardArticleTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_u32()?,
            r#text: row.field(1usize + offset)?.into_string()?,
            r#npc_name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
