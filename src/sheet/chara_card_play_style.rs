use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CharaCardPlayStyle {
    fn name() -> String {
        "CharaCardPlayStyle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaCardPlayStyle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaCardPlayStyle {
    pub r#icon: i32,
    pub r#sort_key: u8,
    pub r#name: SeString,
}
impl CharaCardPlayStyle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#sort_key: row.field(1usize + offset)?.into_u8()?,
            r#name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
