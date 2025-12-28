use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#class_job: bool,
    pub r#unknown3: u8,
    pub r#level: u8,
    pub r#quest: u8,
    pub r#value: u32,
    pub r#class_job_category: i16,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
}
impl Trait {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#class_job: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#level: row.field(4usize + offset)?.into_u8()?,
            r#quest: row.field(5usize + offset)?.into_u8()?,
            r#value: row.field(6usize + offset)?.into_u32()?,
            r#class_job_category: row.field(7usize + offset)?.into_i16()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
        })
    }
}
