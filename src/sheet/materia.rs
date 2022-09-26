use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::utility::read_array;
use crate::error::PopulateError;
use std::vec::Vec;
use std::result::Result;
impl MetadataAdapter for Materia {
    fn name() -> String {
        "Materia".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Materia::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Materia {
    pub r#item: Vec<i32>,
    pub r#base_param: u8,
    pub r#value: Vec<i16>,
}
impl Materia {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_i32()?) },
            )?,
            r#base_param: row.field(10usize + offset)?.into_u8()?,
            r#value: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(11usize + offset)?.into_i16()?) },
            )?,
        })
    }
}
