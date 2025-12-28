use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CircleActivity {
    fn name() -> String {
        "CircleActivity".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CircleActivity::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CircleActivity {
    pub r#name: SeString,
    pub r#icon: i32,
    pub r#order: u16,
}
impl CircleActivity {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#order: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
