use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CompleteJournal {
    fn name() -> String {
        "CompleteJournal".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompleteJournal::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompleteJournal {
    pub r#unknown0: u32,
    pub r#required_level: u16,
    pub r#unknown2: u8,
    pub r#icon: i32,
    pub r#unknown4: u32,
    pub r#name: SeString,
    pub r#cutscene: Vec<i32>,
}
impl CompleteJournal {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#required_level: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#icon: row.field(3usize + offset)?.into_i32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#name: row.field(5usize + offset)?.into_string()?,
            r#cutscene: read_array(
                offset,
                24usize,
                1usize,
                |offset| { Result::Ok(row.field(6usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
