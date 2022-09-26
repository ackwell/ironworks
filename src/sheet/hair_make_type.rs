use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for HairMakeType {
    fn name() -> String {
        "HairMakeType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HairMakeType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HairMakeType {
    pub r#race: i32,
    pub r#tribe: i32,
    pub r#gender: i8,
}
impl HairMakeType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#race: row.field(0usize + offset)?.into_i32()?,
            r#tribe: row.field(1usize + offset)?.into_i32()?,
            r#gender: row.field(2usize + offset)?.into_i8()?,
        })
    }
}
