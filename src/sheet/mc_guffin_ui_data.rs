use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for McGuffinUIData {
    fn name() -> String {
        "McGuffinUIData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(McGuffinUIData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct McGuffinUIData {
    pub r#order: u16,
    pub r#icon: u32,
    pub r#name: SeString,
}
impl McGuffinUIData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#order: row.field(0usize + offset)?.into_u16()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
