use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Picture {
    fn name() -> String {
        "Picture".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Picture::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Picture {
    pub r#image: i32,
    pub r#signature: i32,
}
impl Picture {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
            r#signature: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
