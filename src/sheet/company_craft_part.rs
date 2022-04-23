use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use std::vec::Vec;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for CompanyCraftPart {
    fn name() -> String {
        "CompanyCraftPart".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftPart::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftPart {
    pub r#unknown0: u8,
    pub r#company_craft_type: u8,
    pub r#company_craft_process: Vec<u16>,
}
impl CompanyCraftPart {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#company_craft_type: row.field(1usize + offset)?.into_u8()?,
            r#company_craft_process: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
