use std::result::Result;
use std::convert::Infallible;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
    pub r#item: i32,
    pub r#image: i32,
    pub r#signature: Option<Infallible>,
}
impl Picture {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#image: row.field(1usize + offset)?.into_i32()?,
            r#signature: None,
        })
    }
}
