use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for AOZScore {
    fn name() -> String {
        "AOZScore".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZScore::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZScore {
    pub r#is_hidden: bool,
    pub r#score: i32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl AOZScore {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#is_hidden: row.field(0usize + offset)?.into_bool()?,
            r#score: row.field(1usize + offset)?.into_i32()?,
            r#name: row.field(2usize + offset)?.into_string()?,
            r#description: row.field(3usize + offset)?.into_string()?,
        })
    }
}
