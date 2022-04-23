use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for LeveVfx {
    fn name() -> String {
        "LeveVfx".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LeveVfx::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LeveVfx {
    pub r#effect: SeString,
    pub r#icon: i32,
}
impl LeveVfx {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#effect: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
