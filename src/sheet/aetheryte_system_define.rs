use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for AetheryteSystemDefine {
    fn name() -> String {
        "AetheryteSystemDefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AetheryteSystemDefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AetheryteSystemDefine {
    pub r#text: SeString,
    pub r#define_value: u32,
}
impl AetheryteSystemDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#define_value: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
