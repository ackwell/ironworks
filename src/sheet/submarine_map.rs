use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for SubmarineMap {
    fn name() -> String {
        "SubmarineMap".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SubmarineMap::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SubmarineMap {
    pub r#name: SeString,
    pub r#image: u32,
}
impl SubmarineMap {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#image: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
