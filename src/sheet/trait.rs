use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for Trait {
    fn name() -> String {
        "Trait".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Trait::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Trait {
    pub r#name: SeString,
    pub r#icon: i32,
    pub r#class_job: u8,
    pub r#level: u8,
    pub r#quest: u32,
    pub r#value: i16,
    pub r#class_job_category: u8,
}
impl Trait {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#class_job: row.field(2usize + offset)?.into_u8()?,
            r#level: row.field(3usize + offset)?.into_u8()?,
            r#quest: row.field(4usize + offset)?.into_u32()?,
            r#value: row.field(5usize + offset)?.into_i16()?,
            r#class_job_category: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
