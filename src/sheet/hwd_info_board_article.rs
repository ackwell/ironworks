use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for HWDInfoBoardArticle {
    fn name() -> String {
        "HWDInfoBoardArticle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDInfoBoardArticle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDInfoBoardArticle {
    pub r#type: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u16,
    pub r#unknown3: bool,
    pub r#text: SeString,
}
impl HWDInfoBoardArticle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#text: row.field(4usize + offset)?.into_string()?,
        })
    }
}
