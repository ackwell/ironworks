use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MainCommandCategory {
    fn name() -> String {
        "MainCommandCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MainCommandCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MainCommandCategory {
    pub r#unknown0: i32,
    pub r#name: SeString,
}
impl MainCommandCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
